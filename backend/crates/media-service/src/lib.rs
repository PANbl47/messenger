pub mod upload_tickets;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use media_pipeline::{StorageBudget, StorageGuardError};

pub use upload_tickets::{UploadTicket, UploadTicketManager};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentDraft {
    pub content_type: String,
    pub size_bytes: u64,
}

impl AttachmentDraft {
    pub fn new(content_type: impl Into<String>, size_bytes: u64) -> Self {
        Self {
            content_type: content_type.into(),
            size_bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewAttachmentMessage {
    pub logical_id: String,
    pub conversation_id: String,
    pub author_device_id: String,
    pub text_body: String,
    pub attachment: AttachmentDraft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentUploadStatus {
    WaitingForUpload,
    Uploading,
    Uploaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceDraftState {
    Draft,
    Sending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentMessageRecord {
    pub logical_id: String,
    pub conversation_id: String,
    pub author_device_id: String,
    pub text_body: String,
    pub attachment: AttachmentDraft,
    pub attachment_status: AttachmentUploadStatus,
    pub active_ticket: Option<UploadTicket>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceDraftRecord {
    pub logical_id: String,
    pub conversation_id: String,
    pub encoded_audio: Vec<u8>,
    pub waveform: Vec<u8>,
    pub state: VoiceDraftState,
    pub editable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaServiceError {
    UnknownMessage,
    UnknownVoiceDraft,
    StorageFull,
}

#[derive(Debug, Clone)]
pub struct MediaService {
    inner: Arc<Mutex<MediaState>>,
}

#[derive(Debug)]
struct MediaState {
    storage_budget: StorageBudget,
    upload_tickets: UploadTicketManager,
    attachment_messages: HashMap<String, AttachmentMessageRecord>,
    conversations: HashMap<String, Vec<String>>,
    voice_drafts: HashMap<String, VoiceDraftRecord>,
}

impl MediaService {
    pub fn new() -> Self {
        Self::with_storage_capacity_bytes(u64::MAX)
    }

    pub fn with_storage_capacity_bytes(capacity_bytes: u64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(MediaState {
                storage_budget: StorageBudget::new(capacity_bytes),
                upload_tickets: UploadTicketManager::default(),
                attachment_messages: HashMap::new(),
                conversations: HashMap::new(),
                voice_drafts: HashMap::new(),
            })),
        }
    }

    pub async fn send_attachment(
        &self,
        message: NewAttachmentMessage,
    ) -> Result<AttachmentMessageRecord, MediaServiceError> {
        let mut state = self.inner.lock().expect("media state mutex poisoned");

        if let Some(existing) = state.attachment_messages.get(&message.logical_id) {
            return Ok(existing.clone());
        }

        state
            .storage_budget
            .reserve(message.attachment.size_bytes)
            .map_err(map_storage_error)?;

        let active_ticket = Some(state.upload_tickets.issue(&message.logical_id));
        let record = AttachmentMessageRecord {
            logical_id: message.logical_id.clone(),
            conversation_id: message.conversation_id.clone(),
            author_device_id: message.author_device_id,
            text_body: message.text_body,
            attachment: message.attachment,
            attachment_status: AttachmentUploadStatus::Uploading,
            active_ticket,
        };

        state
            .conversations
            .entry(record.conversation_id.clone())
            .or_default()
            .push(record.logical_id.clone());
        state
            .attachment_messages
            .insert(record.logical_id.clone(), record.clone());

        Ok(record)
    }

    pub async fn interrupt_upload(
        &self,
        logical_id: &str,
    ) -> Result<UploadTicket, MediaServiceError> {
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        let record = state
            .attachment_messages
            .get_mut(logical_id)
            .ok_or(MediaServiceError::UnknownMessage)?;
        let ticket = record
            .active_ticket
            .take()
            .ok_or(MediaServiceError::UnknownMessage)?;
        record.attachment_status = AttachmentUploadStatus::WaitingForUpload;
        Ok(ticket)
    }

    pub async fn expire_active_ticket(&self, logical_id: &str) {
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        if let Some(record) = state.attachment_messages.get_mut(logical_id) {
            if let Some(ticket) = record.active_ticket.as_mut() {
                ticket.expires_at_epoch_ms = 0;
            }
            record.attachment_status = AttachmentUploadStatus::WaitingForUpload;
        }
    }

    pub async fn resume_attachment_upload(
        &self,
        logical_id: &str,
    ) -> Result<AttachmentMessageRecord, MediaServiceError> {
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        let new_ticket = state.upload_tickets.issue(logical_id);
        let record = state
            .attachment_messages
            .get_mut(logical_id)
            .ok_or(MediaServiceError::UnknownMessage)?;
        record.active_ticket = Some(new_ticket);
        record.attachment_status = AttachmentUploadStatus::WaitingForUpload;
        Ok(record.clone())
    }

    pub async fn timeline(&self, conversation_id: &str) -> Vec<AttachmentMessageRecord> {
        let state = self.inner.lock().expect("media state mutex poisoned");
        state
            .conversations
            .get(conversation_id)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|logical_id| state.attachment_messages.get(logical_id).cloned())
            .collect()
    }

    pub async fn save_voice_draft(
        &self,
        logical_id: &str,
        conversation_id: &str,
        encoded_audio: Vec<u8>,
    ) {
        let draft = VoiceDraftRecord {
            logical_id: logical_id.to_string(),
            conversation_id: conversation_id.to_string(),
            waveform: encoded_audio.clone(),
            encoded_audio,
            state: VoiceDraftState::Draft,
            editable: true,
        };
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        state.voice_drafts.insert(logical_id.to_string(), draft);
    }

    pub async fn begin_voice_send(&self, logical_id: &str) -> Result<(), MediaServiceError> {
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        let draft = state
            .voice_drafts
            .get_mut(logical_id)
            .ok_or(MediaServiceError::UnknownVoiceDraft)?;
        draft.state = VoiceDraftState::Sending;
        draft.editable = false;
        Ok(())
    }

    pub async fn cancel_voice_send(&self, logical_id: &str) -> Result<(), MediaServiceError> {
        let mut state = self.inner.lock().expect("media state mutex poisoned");
        let draft = state
            .voice_drafts
            .get_mut(logical_id)
            .ok_or(MediaServiceError::UnknownVoiceDraft)?;
        draft.state = VoiceDraftState::Draft;
        draft.editable = true;
        Ok(())
    }

    pub async fn voice_draft(
        &self,
        logical_id: &str,
    ) -> Result<VoiceDraftRecord, MediaServiceError> {
        let state = self.inner.lock().expect("media state mutex poisoned");
        state
            .voice_drafts
            .get(logical_id)
            .cloned()
            .ok_or(MediaServiceError::UnknownVoiceDraft)
    }
}

fn map_storage_error(_error: StorageGuardError) -> MediaServiceError {
    MediaServiceError::StorageFull
}
