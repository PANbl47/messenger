use message_protocol::DraftPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationId(String);

impl ConversationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLifecycle {
    Draft,
    LocalPending,
    Encrypted,
    Queued,
    Sending,
    ServerAccepted,
    Delivered,
    Read,
    FailedRetryable,
    FailedTerminal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftMessage {
    conversation_id: ConversationId,
    payload: DraftPayload,
    lifecycle: MessageLifecycle,
}

impl DraftMessage {
    pub fn new(conversation_id: ConversationId, body: impl Into<String>) -> Self {
        Self {
            conversation_id,
            payload: DraftPayload::text(body),
            lifecycle: MessageLifecycle::Draft,
        }
    }

    pub fn lifecycle(&self) -> MessageLifecycle {
        self.lifecycle
    }

    pub fn advance_to(&self, next: MessageLifecycle) -> Result<Self, MessageStateError> {
        if self.lifecycle.can_transition_to(next) {
            let mut updated = self.clone();
            updated.lifecycle = next;
            Ok(updated)
        } else {
            Err(MessageStateError::InvalidTransition {
                from: self.lifecycle,
                to: next,
            })
        }
    }
}

impl MessageLifecycle {
    fn can_transition_to(self, next: MessageLifecycle) -> bool {
        matches!(
            (self, next),
            (Self::Draft, Self::LocalPending)
                | (Self::LocalPending, Self::Encrypted)
                | (Self::Encrypted, Self::Queued)
                | (Self::Queued, Self::Sending)
                | (Self::Queued, Self::FailedRetryable)
                | (Self::FailedRetryable, Self::Queued)
                | (Self::Sending, Self::ServerAccepted)
                | (Self::Sending, Self::FailedRetryable)
                | (Self::Sending, Self::FailedTerminal)
                | (Self::ServerAccepted, Self::Delivered)
                | (Self::Delivered, Self::Read)
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStateError {
    InvalidTransition {
        from: MessageLifecycle,
        to: MessageLifecycle,
    },
}
