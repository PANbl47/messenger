use media_service::{
    AttachmentDraft, AttachmentUploadStatus, MediaService, MediaServiceError, NewAttachmentMessage,
};

fn photo_message() -> NewAttachmentMessage {
    NewAttachmentMessage {
        logical_id: "logical-photo-1".to_string(),
        conversation_id: "chat-alpha".to_string(),
        author_device_id: "device-a".to_string(),
        text_body: "photo".to_string(),
        attachment: AttachmentDraft::new("image/jpeg", 512 * 1024),
    }
}

#[tokio::test]
async fn interrupted_attachment_upload_retries_without_duplicate_message_creation() {
    let service = MediaService::new();

    let started = service
        .send_attachment(photo_message())
        .await
        .expect("attachment send should start");
    let interrupted_ticket = service
        .interrupt_upload("logical-photo-1")
        .await
        .expect("upload ticket should exist");

    let resumed = service
        .resume_attachment_upload("logical-photo-1")
        .await
        .expect("interrupted upload should renew its ticket");

    assert_eq!(started.logical_id, resumed.logical_id);
    assert_eq!(service.timeline("chat-alpha").await.len(), 1);
    assert_ne!(interrupted_ticket.id, resumed.active_ticket.expect("ticket").id);
    assert_eq!(
        resumed.attachment_status,
        AttachmentUploadStatus::WaitingForUpload
    );
}

#[tokio::test]
async fn expired_upload_tickets_are_renewed_before_message_failure() {
    let service = MediaService::new();
    service
        .send_attachment(photo_message())
        .await
        .expect("attachment send should start");
    service.expire_active_ticket("logical-photo-1").await;

    let resumed = service
        .resume_attachment_upload("logical-photo-1")
        .await
        .expect("expired upload should renew its ticket");

    assert_eq!(service.timeline("chat-alpha").await.len(), 1);
    assert!(resumed.active_ticket.expect("ticket").expires_at_epoch_ms > 0);
    assert_eq!(
        resumed.attachment_status,
        AttachmentUploadStatus::WaitingForUpload
    );
}

#[tokio::test]
async fn storage_full_prevents_queuing_attachment_message() {
    let service = MediaService::with_storage_capacity_bytes(32);

    let error = service
        .send_attachment(photo_message())
        .await
        .expect_err("storage full should fail locally before queueing");

    assert_eq!(error, MediaServiceError::StorageFull);
    assert!(service.timeline("chat-alpha").await.is_empty());
}
