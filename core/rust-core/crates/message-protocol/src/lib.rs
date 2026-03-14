use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftPayload {
    Text(TextDraft),
}

impl DraftPayload {
    pub fn text(body: impl Into<String>) -> Self {
        Self::Text(TextDraft::new(body))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDraft {
    pub body: String,
}

impl TextDraft {
    pub fn new(body: impl Into<String>) -> Self {
        Self { body: body.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::DraftPayload;

    #[test]
    fn draft_payload_round_trips_through_serde() {
        let payload = DraftPayload::text("hello");
        let json = serde_json::to_string(&payload).unwrap();
        let restored: DraftPayload = serde_json::from_str(&json).unwrap();

        assert_eq!(restored, payload);
    }
}
