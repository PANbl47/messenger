use media_service::{MediaService, VoiceDraftState};

#[tokio::test]
async fn cancelled_voice_send_returns_to_editable_draft_state() {
    let service = MediaService::new();

    service
        .save_voice_draft("voice-1", "chat-alpha", vec![1, 2, 3, 4])
        .await;
    service.begin_voice_send("voice-1").await.expect("draft exists");
    service
        .cancel_voice_send("voice-1")
        .await
        .expect("voice send can be cancelled");

    let draft = service
        .voice_draft("voice-1")
        .await
        .expect("voice draft should be recoverable");

    assert_eq!(draft.state, VoiceDraftState::Draft);
    assert!(draft.editable);
    assert_eq!(draft.waveform, vec![1, 2, 3, 4]);
}
