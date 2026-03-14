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
pub enum DeviceTrustState {
    Untrusted,
    Trusted,
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
