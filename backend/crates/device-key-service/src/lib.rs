pub mod enrollment;
pub mod recovery;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use device_trust::{
    DeviceRecord, DeviceTrustState, EnrollmentDecision, HistoryAccess, PasswordRecoveryStatus,
    WrappedKeyRef,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddDeviceRequest {
    pub account_id: String,
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub wrapped_account_key_ref: Option<String>,
    pub approving_device_id: Option<String>,
    pub last_active_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryEnrollmentRequest {
    pub account_id: String,
    pub device_id: String,
    pub recovery_wrap_ref: Option<String>,
    pub last_active_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoveDeviceRequest {
    pub account_id: String,
    pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceKeyError {
    ApprovalRequired,
    DeviceNotFound,
    RecoveryUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryEnrollmentResult {
    pub device: DeviceRecord,
    pub history_access_guaranteed: bool,
    pub password_recovery_status: PasswordRecoveryStatus,
}

#[derive(Debug, Clone, Default)]
pub struct DeviceKeyService {
    inner: Arc<Mutex<HashMap<String, AccountDevices>>>,
}

#[derive(Debug, Clone)]
struct AccountDevices {
    devices: Vec<DeviceRecord>,
    password_recovery_wrap: Option<WrappedKeyRef>,
    password_recovery_status: PasswordRecoveryStatus,
}

impl Default for AccountDevices {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
            password_recovery_wrap: None,
            password_recovery_status: PasswordRecoveryStatus::NotConfigured,
        }
    }
}

impl DeviceKeyService {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn add_device(
        &self,
        request: AddDeviceRequest,
    ) -> Result<DeviceRecord, DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state.entry(request.account_id).or_default();

        let has_trusted = account
            .devices
            .iter()
            .any(|device| device.trust_state == DeviceTrustState::Trusted);

        let trust_state = if !has_trusted {
            trusted(EnrollmentDecision::Trusted)
        } else if request.approving_device_id.as_deref().is_some_and(|approver| {
            account
                .devices
                .iter()
                .any(|device| {
                    device.device_id == approver && device.trust_state == DeviceTrustState::Trusted
                })
        }) {
            trusted(EnrollmentDecision::Trusted)
        } else {
            return Err(DeviceKeyError::ApprovalRequired);
        };

        let record = DeviceRecord::new(
            request.device_id,
            request.device_name,
            request.device_type,
            trust_state,
            request.wrapped_account_key_ref.is_some(),
            request.last_active_at,
        );

        account.devices.push(record.clone());
        Ok(record)
    }

    pub async fn recover_without_trusted_device(
        &self,
        request: RecoveryEnrollmentRequest,
    ) -> Result<RecoveryEnrollmentResult, DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state.entry(request.account_id).or_default();

        let password_recovery_status = match (
            account.password_recovery_status,
            account.password_recovery_wrap.as_ref(),
            request.recovery_wrap_ref.as_deref(),
        ) {
            (PasswordRecoveryStatus::Invalidated, _, _) => PasswordRecoveryStatus::Invalidated,
            (PasswordRecoveryStatus::Active, Some(expected), Some(provided))
                if expected == &WrappedKeyRef::new(provided) =>
            {
                PasswordRecoveryStatus::Active
            }
            (PasswordRecoveryStatus::Active, _, _) => PasswordRecoveryStatus::Invalidated,
            _ => PasswordRecoveryStatus::NotConfigured,
        };

        let device = DeviceRecord::new(
            request.device_id,
            "Recovered Device",
            "recovery",
            trusted(EnrollmentDecision::RecoveryOnly),
            false,
            request.last_active_at,
        );

        account.devices.push(device.clone());

        Ok(RecoveryEnrollmentResult {
            device,
            history_access_guaranteed: false,
            password_recovery_status,
        })
    }

    pub async fn remove_device(
        &self,
        request: RemoveDeviceRequest,
    ) -> Result<(), DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state
            .get_mut(&request.account_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;

        let device = account
            .devices
            .iter_mut()
            .find(|device| device.device_id == request.device_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;

        device.trust_state = DeviceTrustState::Revoked;
        device.has_wrapped_account_material = false;
        Ok(())
    }

    pub async fn list_devices(&self, account_id: &str) -> Result<Vec<DeviceRecord>, DeviceKeyError> {
        let state = self.inner.lock().expect("device state mutex poisoned");
        let account = state
            .get(account_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;
        Ok(account.devices.clone())
    }

    pub async fn set_password_recovery_material(
        &self,
        account_id: &str,
        wrapped_key_ref: WrappedKeyRef,
    ) -> Result<(), DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state.entry(account_id.to_string()).or_default();
        account.password_recovery_wrap = Some(wrapped_key_ref);
        account.password_recovery_status = PasswordRecoveryStatus::Active;
        Ok(())
    }

    pub async fn reset_password(&self, account_id: &str) -> Result<(), DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state.entry(account_id.to_string()).or_default();
        account.password_recovery_status = PasswordRecoveryStatus::Invalidated;
        account.password_recovery_wrap = None;
        Ok(())
    }

    pub async fn history_access(
        &self,
        account_id: &str,
        device_id: &str,
    ) -> Result<HistoryAccess, DeviceKeyError> {
        let state = self.inner.lock().expect("device state mutex poisoned");
        let account = state
            .get(account_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;
        let device = account
            .devices
            .iter()
            .find(|device| device.device_id == device_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;
        Ok(HistoryAccess::from_device(device))
    }

    pub async fn record_last_active(
        &self,
        account_id: &str,
        device_id: &str,
        last_active_at: u64,
    ) -> Result<(), DeviceKeyError> {
        let mut state = self.inner.lock().expect("device state mutex poisoned");
        let account = state
            .get_mut(account_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;
        let device = account
            .devices
            .iter_mut()
            .find(|device| device.device_id == device_id)
            .ok_or(DeviceKeyError::DeviceNotFound)?;
        device.last_active_at = last_active_at;
        Ok(())
    }
}

pub fn trusted(decision: EnrollmentDecision) -> DeviceTrustState {
    match decision {
        EnrollmentDecision::Trusted => DeviceTrustState::Trusted,
        EnrollmentDecision::PendingApproval => DeviceTrustState::PendingApproval,
        EnrollmentDecision::RecoveryOnly => DeviceTrustState::RecoveryOnly,
    }
}
