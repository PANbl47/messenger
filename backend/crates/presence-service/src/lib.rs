use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutgoingSignal {
    MessageDelivery {
        logical_id: String,
        conversation_id: String,
    },
    PresenceUpdate {
        conversation_id: String,
        device_id: String,
        is_typing: bool,
    },
}

#[derive(Debug, Clone)]
pub struct PresenceService {
    inner: Arc<Mutex<PresenceState>>,
}

#[derive(Debug)]
struct PresenceState {
    max_presence_per_drain: usize,
    message_queue: VecDeque<OutgoingSignal>,
    presence_order: VecDeque<PresenceKey>,
    presence_by_key: HashMap<PresenceKey, OutgoingSignal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PresenceKey {
    conversation_id: String,
    device_id: String,
}

impl PresenceService {
    pub fn new(max_presence_per_drain: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(PresenceState {
                max_presence_per_drain,
                message_queue: VecDeque::new(),
                presence_order: VecDeque::new(),
                presence_by_key: HashMap::new(),
            })),
        }
    }

    pub async fn enqueue_presence(
        &self,
        conversation_id: &str,
        device_id: &str,
        is_typing: bool,
    ) {
        let mut state = self.inner.lock().expect("presence state mutex poisoned");
        let key = PresenceKey {
            conversation_id: conversation_id.to_string(),
            device_id: device_id.to_string(),
        };
        if !state.presence_by_key.contains_key(&key) {
            state.presence_order.push_back(key.clone());
        }
        state.presence_by_key.insert(
            key,
            OutgoingSignal::PresenceUpdate {
                conversation_id: conversation_id.to_string(),
                device_id: device_id.to_string(),
                is_typing,
            },
        );
    }

    pub async fn enqueue_message_delivery(&self, logical_id: &str, conversation_id: &str) {
        let mut state = self.inner.lock().expect("presence state mutex poisoned");
        state
            .message_queue
            .push_back(OutgoingSignal::MessageDelivery {
                logical_id: logical_id.to_string(),
                conversation_id: conversation_id.to_string(),
            });
    }

    pub async fn drain_transport(&self, max_items: usize) -> Vec<OutgoingSignal> {
        let mut state = self.inner.lock().expect("presence state mutex poisoned");
        let mut batch = Vec::new();

        while batch.len() < max_items {
            let Some(next_message) = state.message_queue.pop_front() else {
                break;
            };
            batch.push(next_message);
        }

        let remaining = max_items.saturating_sub(batch.len());
        let presence_budget = remaining.min(state.max_presence_per_drain);
        for _ in 0..presence_budget {
            let Some(key) = state.presence_order.pop_front() else {
                break;
            };
            if let Some(signal) = state.presence_by_key.remove(&key) {
                batch.push(signal);
            }
        }

        batch
    }

    pub async fn pending_presence_count(&self) -> usize {
        let state = self.inner.lock().expect("presence state mutex poisoned");
        state.presence_by_key.len()
    }
}
