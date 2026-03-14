use messaging_service::{
    DeleteMessage, EditMessage, ForwardMessage, MessagingService, NewTextMessage,
    SyncEvent, SyncEventKind,
};

fn text_message(logical_id: &str, body: &str) -> NewTextMessage {
    NewTextMessage {
        logical_id: logical_id.to_string(),
        conversation_id: "chat-sync".to_string(),
        author_device_id: "device-a".to_string(),
        body: body.to_string(),
    }
}

#[tokio::test]
async fn edit_delete_reply_and_forward_events_sync_to_other_devices() {
    let service = MessagingService::new();
    service.set_network_available(true).await;
    let mut device_b = service.subscribe("device-b").await;

    let original = service.send_text(text_message("logical-1", "hello")).await;
    let edited = service
        .edit_message(EditMessage {
            logical_id: original.logical_id.clone(),
            edited_body: "hello, edited".to_string(),
        })
        .await;
    let reply = service.reply_text(text_message("logical-2", "reply"), &original.logical_id).await;
    let forward = service
        .forward_text(ForwardMessage {
            logical_id: "logical-3".to_string(),
            conversation_id: "chat-sync".to_string(),
            author_device_id: "device-a".to_string(),
            body: "forward".to_string(),
            forward_from: original.logical_id.clone(),
        })
        .await;
    let deleted = service
        .delete_message(DeleteMessage {
            logical_id: original.logical_id.clone(),
        })
        .await;

    assert_eq!(edited.body, "hello, edited");
    assert_eq!(reply.reply_to.as_deref(), Some(original.logical_id.as_str()));
    assert_eq!(forward.forward_from.as_deref(), Some(original.logical_id.as_str()));
    assert!(deleted.deleted);

    let received = receive_events(&mut device_b, 5).await;
    assert_eq!(
        received.iter().map(|event| &event.kind).collect::<Vec<_>>(),
        vec![
            &SyncEventKind::MessageSent {
                logical_id: "logical-1".to_string(),
                body: "hello".to_string(),
            },
            &SyncEventKind::MessageEdited {
                logical_id: "logical-1".to_string(),
                body: "hello, edited".to_string(),
            },
            &SyncEventKind::MessageSent {
                logical_id: "logical-2".to_string(),
                body: "reply".to_string(),
            },
            &SyncEventKind::MessageSent {
                logical_id: "logical-3".to_string(),
                body: "forward".to_string(),
            },
            &SyncEventKind::MessageDeleted {
                logical_id: "logical-1".to_string(),
            },
        ]
    );
}

#[tokio::test]
async fn websocket_subscriber_receives_message_edit_and_delete_events_in_persisted_order() {
    let service = MessagingService::new();
    service.set_network_available(true).await;
    let mut device_b = service.subscribe("device-b").await;

    let original = service.send_text(text_message("logical-order", "ordered")).await;
    service
        .edit_message(EditMessage {
            logical_id: original.logical_id.clone(),
            edited_body: "ordered again".to_string(),
        })
        .await;
    service
        .delete_message(DeleteMessage {
            logical_id: original.logical_id.clone(),
        })
        .await;

    let persisted = service.persisted_events().await;
    let websocket = receive_events(&mut device_b, 3).await;

    assert_eq!(persisted, websocket);
}

async fn receive_events(
    receiver: &mut tokio::sync::broadcast::Receiver<SyncEvent>,
    count: usize,
) -> Vec<SyncEvent> {
    let mut events = Vec::with_capacity(count);
    while events.len() < count {
        events.push(receiver.recv().await.expect("subscriber should receive event"));
    }
    events
}
