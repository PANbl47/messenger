use presence_service::{OutgoingSignal, PresenceService};

#[tokio::test]
async fn presence_updates_do_not_block_message_delivery() {
    let service = PresenceService::new(2);

    service.enqueue_presence("chat-alpha", "device-a", true).await;
    service.enqueue_presence("chat-alpha", "device-a", true).await;
    service
        .enqueue_message_delivery("message-1", "chat-alpha")
        .await;

    let batch = service.drain_transport(1).await;

    assert_eq!(batch, vec![OutgoingSignal::MessageDelivery {
        logical_id: "message-1".to_string(),
        conversation_id: "chat-alpha".to_string(),
    }]);
    assert_eq!(service.pending_presence_count().await, 1);
}
