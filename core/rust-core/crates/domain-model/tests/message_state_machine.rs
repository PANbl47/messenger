use domain_model::{ConversationId, DraftMessage, MessageLifecycle, MessageStateError};

#[test]
fn message_state_transitions_allow_retryable_failure_only_after_queueing() {
    let draft = DraftMessage::new(ConversationId::new("chat_1"), "queued hello");

    let local_pending = draft.advance_to(MessageLifecycle::LocalPending).unwrap();
    assert_eq!(local_pending.lifecycle(), MessageLifecycle::LocalPending);

    let encrypted = local_pending
        .advance_to(MessageLifecycle::Encrypted)
        .unwrap();
    assert_eq!(encrypted.lifecycle(), MessageLifecycle::Encrypted);

    let queued = encrypted.advance_to(MessageLifecycle::Queued).unwrap();
    assert_eq!(queued.lifecycle(), MessageLifecycle::Queued);

    let failed = queued
        .advance_to(MessageLifecycle::FailedRetryable)
        .unwrap();
    assert_eq!(failed.lifecycle(), MessageLifecycle::FailedRetryable);

    let premature = DraftMessage::new(ConversationId::new("chat_2"), "too early")
        .advance_to(MessageLifecycle::FailedRetryable)
        .unwrap_err();

    assert_eq!(
        premature,
        MessageStateError::InvalidTransition {
            from: MessageLifecycle::Draft,
            to: MessageLifecycle::FailedRetryable,
        }
    );
}

#[test]
fn message_contracts_round_trip_through_serde() {
    let draft = DraftMessage::new(ConversationId::new("chat_serde"), "payload");
    let json = serde_json::to_string(&draft).unwrap();
    let restored: DraftMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, draft);
}
