use std::time::Duration;

use messaging_service::{DeliveryStatus, MessageRecord, MessagingService, NewTextMessage};

fn text_message(logical_id: &str, body: &str) -> NewTextMessage {
    NewTextMessage {
        logical_id: logical_id.to_string(),
        conversation_id: "chat-alpha".to_string(),
        author_device_id: "device-a".to_string(),
        body: body.to_string(),
    }
}

#[tokio::test]
async fn queued_message_retries_automatically_after_reconnect() {
    let service = MessagingService::new();
    service.set_network_available(false).await;

    let queued = service.send_text(text_message("logical-1", "hello offline")).await;

    assert_eq!(queued.delivery_status, DeliveryStatus::Queued);

    let timeline = service.timeline("chat-alpha").await;
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].body, "hello offline");

    service.set_network_available(true).await;

    let delivered = service
        .message("logical-1")
        .await
        .expect("message should still exist after reconnect");
    assert_eq!(delivered.delivery_status, DeliveryStatus::Delivered);
}

#[tokio::test]
async fn message_becomes_retryable_failure_after_three_minutes() {
    let service = MessagingService::new();
    service.set_network_available(false).await;
    service.send_text(text_message("logical-2", "stuck message")).await;

    service.advance_time(Duration::from_secs(181)).await;

    let failed = service
        .message("logical-2")
        .await
        .expect("message should remain visible in the timeline");

    assert_eq!(failed.delivery_status, DeliveryStatus::FailedRetryable);
    assert_timeline_keeps_failed_message(&service, failed).await;
}

async fn assert_timeline_keeps_failed_message(service: &MessagingService, failed: MessageRecord) {
    let timeline = service.timeline("chat-alpha").await;
    assert_eq!(timeline, vec![failed]);
}

#[tokio::test]
async fn duplicate_logical_messages_do_not_create_duplicate_timeline_entries() {
    let service = MessagingService::new();
    service.set_network_available(false).await;

    let first = service.send_text(text_message("logical-dup", "same logical message")).await;
    let second = service.send_text(text_message("logical-dup", "same logical message")).await;

    assert_eq!(first.logical_id, second.logical_id);
    assert_eq!(service.timeline("chat-alpha").await.len(), 1);
}
