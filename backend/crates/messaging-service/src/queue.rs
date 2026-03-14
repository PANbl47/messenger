use std::time::Duration;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;
use sync_engine::{RetryPolicy, RetryState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryStatus {
    Queued,
    Delivered,
    FailedRetryable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageRecord {
    pub logical_id: String,
    pub conversation_id: String,
    pub author_device_id: String,
    pub body: String,
    pub delivery_status: DeliveryStatus,
    pub reply_to: Option<String>,
    pub forward_from: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Clone)]
pub struct NewTextMessage {
    pub logical_id: String,
    pub conversation_id: String,
    pub author_device_id: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct EditMessage {
    pub logical_id: String,
    pub edited_body: String,
}

#[derive(Debug, Clone)]
pub struct DeleteMessage {
    pub logical_id: String,
}

#[derive(Debug, Clone)]
pub struct ForwardMessage {
    pub logical_id: String,
    pub conversation_id: String,
    pub author_device_id: String,
    pub body: String,
    pub forward_from: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncEvent {
    pub sequence: u64,
    pub kind: SyncEventKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncEventKind {
    MessageSent { logical_id: String, body: String },
    MessageEdited { logical_id: String, body: String },
    MessageDeleted { logical_id: String },
}

#[derive(Debug, Clone)]
pub struct MessagingService {
    inner: Arc<Mutex<MessagingState>>,
    sync_events: broadcast::Sender<SyncEvent>,
}

#[derive(Debug, Default)]
struct MessagingState {
    network_available: bool,
    retry_policy: RetryPolicy,
    messages: HashMap<String, StoredMessage>,
    conversations: HashMap<String, Vec<String>>,
    events: Vec<SyncEvent>,
    next_sequence: u64,
}

#[derive(Debug, Clone)]
struct StoredMessage {
    record: MessageRecord,
    queued_for: Duration,
}

impl MessagingService {
    pub fn new() -> Self {
        let (sync_events, _rx) = broadcast::channel(64);
        Self {
            inner: Arc::new(Mutex::new(MessagingState {
                network_available: true,
                retry_policy: RetryPolicy::default(),
                messages: HashMap::new(),
                conversations: HashMap::new(),
                events: Vec::new(),
                next_sequence: 1,
            })),
            sync_events,
        }
    }

    pub async fn set_network_available(&self, available: bool) {
        let pending_events = {
            let mut state = self.inner.lock().expect("messaging state mutex poisoned");
            state.network_available = available;
            if !available {
                Vec::new()
            } else {
                deliver_queued_messages(&mut state)
            }
        };

        for event in pending_events {
            let _ = self.sync_events.send(event);
        }
    }

    pub async fn subscribe(&self, _device_id: &str) -> broadcast::Receiver<SyncEvent> {
        self.sync_events.subscribe()
    }

    pub async fn send_text(&self, message: NewTextMessage) -> MessageRecord {
        let maybe_event = {
            let mut state = self.inner.lock().expect("messaging state mutex poisoned");

            if let Some(existing) = state.messages.get(&message.logical_id) {
                return existing.record.clone();
            }

            let mut record = MessageRecord {
                logical_id: message.logical_id.clone(),
                conversation_id: message.conversation_id.clone(),
                author_device_id: message.author_device_id,
                body: message.body,
                delivery_status: DeliveryStatus::Queued,
                reply_to: None,
                forward_from: None,
                deleted: false,
            };

            let event = if state.network_available {
                record.delivery_status = DeliveryStatus::Delivered;
                Some(push_event(
                    &mut state,
                    SyncEventKind::MessageSent {
                        logical_id: record.logical_id.clone(),
                        body: record.body.clone(),
                    },
                ))
            } else {
                None
            };

            state
                .conversations
                .entry(record.conversation_id.clone())
                .or_default()
                .push(record.logical_id.clone());
            state.messages.insert(
                record.logical_id.clone(),
                StoredMessage {
                    record: record.clone(),
                    queued_for: Duration::ZERO,
                },
            );

            (record, event)
        };

        if let Some(event) = maybe_event.1.clone() {
            let _ = self.sync_events.send(event);
        }

        maybe_event.0
    }

    pub async fn reply_text(
        &self,
        message: NewTextMessage,
        reply_to: &str,
    ) -> MessageRecord {
        let mut record = self.send_text(message).await;
        let mut state = self.inner.lock().expect("messaging state mutex poisoned");
        let stored = state
            .messages
            .get_mut(&record.logical_id)
            .expect("message should have been stored");
        stored.record.reply_to = Some(reply_to.to_string());
        record = stored.record.clone();
        record
    }

    pub async fn forward_text(&self, message: ForwardMessage) -> MessageRecord {
        let mut record = self
            .send_text(NewTextMessage {
                logical_id: message.logical_id,
                conversation_id: message.conversation_id,
                author_device_id: message.author_device_id,
                body: message.body,
            })
            .await;
        let mut state = self.inner.lock().expect("messaging state mutex poisoned");
        let stored = state
            .messages
            .get_mut(&record.logical_id)
            .expect("message should have been stored");
        stored.record.forward_from = Some(message.forward_from);
        record = stored.record.clone();
        record
    }

    pub async fn edit_message(&self, edit: EditMessage) -> MessageRecord {
        let (record, event) = {
            let mut state = self.inner.lock().expect("messaging state mutex poisoned");
            let stored = state
                .messages
                .get_mut(&edit.logical_id)
                .expect("message should exist before edit");
            stored.record.body = edit.edited_body;
            let record = stored.record.clone();
            let event = push_event(
                &mut state,
                SyncEventKind::MessageEdited {
                    logical_id: record.logical_id.clone(),
                    body: record.body.clone(),
                },
            );
            (record, event)
        };

        let _ = self.sync_events.send(event);
        record
    }

    pub async fn delete_message(&self, delete: DeleteMessage) -> MessageRecord {
        let (record, event) = {
            let mut state = self.inner.lock().expect("messaging state mutex poisoned");
            let stored = state
                .messages
                .get_mut(&delete.logical_id)
                .expect("message should exist before delete");
            stored.record.deleted = true;
            let record = stored.record.clone();
            let event = push_event(
                &mut state,
                SyncEventKind::MessageDeleted {
                    logical_id: record.logical_id.clone(),
                },
            );
            (record, event)
        };

        let _ = self.sync_events.send(event);
        record
    }

    pub async fn advance_time(&self, duration: Duration) {
        let mut state = self.inner.lock().expect("messaging state mutex poisoned");
        let network_available = state.network_available;
        let retry_policy = state.retry_policy;
        for stored in state.messages.values_mut() {
            if stored.record.delivery_status == DeliveryStatus::Queued {
                stored.queued_for += duration;
                if matches!(
                    retry_policy.evaluate(stored.queued_for, network_available),
                    RetryState::FailedRetryable
                ) {
                    stored.record.delivery_status = DeliveryStatus::FailedRetryable;
                }
            }
        }
    }

    pub async fn message(&self, logical_id: &str) -> Option<MessageRecord> {
        let state = self.inner.lock().expect("messaging state mutex poisoned");
        state.messages.get(logical_id).map(|stored| stored.record.clone())
    }

    pub async fn timeline(&self, conversation_id: &str) -> Vec<MessageRecord> {
        let state = self.inner.lock().expect("messaging state mutex poisoned");
        state
            .conversations
            .get(conversation_id)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| state.messages.get(id).map(|stored| stored.record.clone()))
            .collect()
    }

    pub async fn persisted_events(&self) -> Vec<SyncEvent> {
        let state = self.inner.lock().expect("messaging state mutex poisoned");
        state.events.clone()
    }
}

fn push_event(state: &mut MessagingState, kind: SyncEventKind) -> SyncEvent {
    let event = SyncEvent {
        sequence: state.next_sequence,
        kind,
    };
    state.next_sequence += 1;
    state.events.push(event.clone());
    event
}

fn deliver_queued_messages(state: &mut MessagingState) -> Vec<SyncEvent> {
    let mut events = Vec::new();
    let queued_ids: Vec<String> = state
        .messages
        .iter()
        .filter_map(|(id, stored)| {
            (stored.record.delivery_status == DeliveryStatus::Queued).then(|| id.clone())
        })
        .collect();

    for logical_id in queued_ids {
        if let Some(stored) = state.messages.get_mut(&logical_id) {
            stored.record.delivery_status = DeliveryStatus::Delivered;
            let body = stored.record.body.clone();
            let event = push_event(
                state,
                SyncEventKind::MessageSent {
                    logical_id: logical_id.clone(),
                    body,
                },
            );
            events.push(event);
        }
    }
    events
}
