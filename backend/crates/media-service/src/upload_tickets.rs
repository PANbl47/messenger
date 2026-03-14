#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadTicket {
    pub id: String,
    pub logical_id: String,
    pub expires_at_epoch_ms: u64,
}

#[derive(Debug, Clone)]
pub struct UploadTicketManager {
    next_generation: u64,
    next_expiry_epoch_ms: u64,
}

impl Default for UploadTicketManager {
    fn default() -> Self {
        Self {
            next_generation: 1,
            next_expiry_epoch_ms: 60_000,
        }
    }
}

impl UploadTicketManager {
    pub fn issue(&mut self, logical_id: &str) -> UploadTicket {
        let generation = self.next_generation;
        let ticket = UploadTicket {
            id: format!("{logical_id}-ticket-{generation}"),
            logical_id: logical_id.to_string(),
            expires_at_epoch_ms: self.next_expiry_epoch_ms,
        };
        self.next_generation += 1;
        self.next_expiry_epoch_ms += 60_000;
        ticket
    }
}
