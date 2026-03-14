#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanupClass {
    QueuedMessage,
    UnsentDraft,
    PendingEdit,
    LocalTrustMaterial,
    EphemeralCache,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupCandidate {
    pub id: String,
    pub class: CleanupClass,
}

impl CleanupCandidate {
    pub fn new(id: impl Into<String>, class: CleanupClass) -> Self {
        Self {
            id: id.into(),
            class,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageGuardError {
    StorageFull {
        required_bytes: u64,
        available_bytes: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageBudget {
    capacity_bytes: u64,
    reserved_bytes: u64,
}

impl StorageBudget {
    pub fn new(capacity_bytes: u64) -> Self {
        Self {
            capacity_bytes,
            reserved_bytes: 0,
        }
    }

    pub fn available_bytes(&self) -> u64 {
        self.capacity_bytes.saturating_sub(self.reserved_bytes)
    }

    pub fn reserve(&mut self, bytes: u64) -> Result<(), StorageGuardError> {
        let available_bytes = self.available_bytes();
        if bytes > available_bytes {
            return Err(StorageGuardError::StorageFull {
                required_bytes: bytes,
                available_bytes,
            });
        }

        self.reserved_bytes = self.reserved_bytes.saturating_add(bytes);
        Ok(())
    }

    pub fn release(&mut self, bytes: u64) {
        self.reserved_bytes = self.reserved_bytes.saturating_sub(bytes);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StorageCleanupGuard;

impl StorageCleanupGuard {
    pub fn filter_deletable(
        &self,
        candidates: Vec<CleanupCandidate>,
    ) -> Vec<CleanupCandidate> {
        candidates
            .into_iter()
            .filter(|candidate| matches!(candidate.class, CleanupClass::EphemeralCache))
            .collect()
    }
}
