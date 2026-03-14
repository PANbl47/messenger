use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceId(String);

impl DeviceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrappedKeyRef(String);

impl WrappedKeyRef {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceTrustState {
    Untrusted,
    PendingApproval,
    Trusted,
    RecoveryOnly,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnrollmentDecision {
    Trusted,
    PendingApproval,
    RecoveryOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasswordRecoveryStatus {
    NotConfigured,
    Active,
    Invalidated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub trust_state: DeviceTrustState,
    pub has_wrapped_account_material: bool,
    pub last_active_at: u64,
}

impl DeviceRecord {
    pub fn new(
        device_id: impl Into<String>,
        device_name: impl Into<String>,
        device_type: impl Into<String>,
        trust_state: DeviceTrustState,
        has_wrapped_account_material: bool,
        last_active_at: u64,
    ) -> Self {
        Self {
            device_id: device_id.into(),
            device_name: device_name.into(),
            device_type: device_type.into(),
            trust_state,
            has_wrapped_account_material,
            last_active_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryAccess {
    pub allowed: bool,
    pub reason: String,
}

impl HistoryAccess {
    pub fn allowed() -> Self {
        Self {
            allowed: true,
            reason: "allowed".to_string(),
        }
    }

    pub fn denied(reason: impl Into<String>) -> Self {
        Self {
            allowed: false,
            reason: reason.into(),
        }
    }

    pub fn from_device(device: &DeviceRecord) -> Self {
        if device.trust_state != DeviceTrustState::Trusted {
            return Self::denied("device_not_authorized");
        }

        if !device.has_wrapped_account_material {
            return Self::denied("missing_wrapped_account_material");
        }

        Self::allowed()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceEnrollment {
    device_id: DeviceId,
    wrapped_key_ref: WrappedKeyRef,
    trust_state: DeviceTrustState,
}

impl DeviceEnrollment {
    pub fn signed_in(device_id: DeviceId, wrapped_key_ref: WrappedKeyRef) -> Self {
        Self {
            device_id,
            wrapped_key_ref,
            trust_state: DeviceTrustState::Untrusted,
        }
    }

    pub fn enroll(mut self) -> Self {
        self.trust_state = DeviceTrustState::Trusted;
        self
    }

    pub fn trust_state(&self) -> DeviceTrustState {
        self.trust_state
    }
}
