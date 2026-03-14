use device_key_service::{AddDeviceRequest, DeviceKeyError, DeviceKeyService, RemoveDeviceRequest};
use device_trust::DeviceTrustState;

#[tokio::test]
async fn second_device_requires_existing_trusted_device_approval_when_available() {
    let service = DeviceKeyService::new();

    let first = service
        .add_device(AddDeviceRequest {
            account_id: "acct-1".into(),
            device_id: "device-1".into(),
            device_name: "Primary Phone".into(),
            device_type: "ios".into(),
            wrapped_account_key_ref: Some("wrap-1".into()),
            approving_device_id: None,
            last_active_at: 100,
        })
        .await
        .expect("first device should enroll");

    assert_eq!(first.trust_state, DeviceTrustState::Trusted);

    let second_attempt = service
        .add_device(AddDeviceRequest {
            account_id: "acct-1".into(),
            device_id: "device-2".into(),
            device_name: "Tablet".into(),
            device_type: "ipad".into(),
            wrapped_account_key_ref: Some("wrap-2".into()),
            approving_device_id: None,
            last_active_at: 200,
        })
        .await;

    assert_eq!(second_attempt, Err(DeviceKeyError::ApprovalRequired));

    let second = service
        .add_device(AddDeviceRequest {
            account_id: "acct-1".into(),
            device_id: "device-2".into(),
            device_name: "Tablet".into(),
            device_type: "ipad".into(),
            wrapped_account_key_ref: Some("wrap-2".into()),
            approving_device_id: Some("device-1".into()),
            last_active_at: 200,
        })
        .await
        .expect("trusted device approval should trust the second device");

    assert_eq!(second.trust_state, DeviceTrustState::Trusted);
}

#[tokio::test]
async fn list_devices_returns_metadata_trust_state_and_last_active_time() {
    let service = DeviceKeyService::new();

    let _ = service
        .add_device(AddDeviceRequest {
            account_id: "acct-2".into(),
            device_id: "device-1".into(),
            device_name: "Calm Phone".into(),
            device_type: "android".into(),
            wrapped_account_key_ref: Some("wrap-1".into()),
            approving_device_id: None,
            last_active_at: 111,
        })
        .await
        .expect("device should enroll");

    service
        .record_last_active("acct-2", "device-1", 222)
        .await
        .expect("last active should update");

    let devices = service
        .list_devices("acct-2")
        .await
        .expect("device list should load");

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_name, "Calm Phone");
    assert_eq!(devices[0].device_type, "android");
    assert_eq!(devices[0].trust_state, DeviceTrustState::Trusted);
    assert_eq!(devices[0].last_active_at, 222);
}

#[tokio::test]
async fn removing_a_device_revokes_future_history_access() {
    let service = DeviceKeyService::new();

    let _ = service
        .add_device(AddDeviceRequest {
            account_id: "acct-3".into(),
            device_id: "device-1".into(),
            device_name: "Desktop".into(),
            device_type: "desktop".into(),
            wrapped_account_key_ref: Some("wrap-1".into()),
            approving_device_id: None,
            last_active_at: 300,
        })
        .await
        .expect("device should enroll");

    service
        .remove_device(RemoveDeviceRequest {
            account_id: "acct-3".into(),
            device_id: "device-1".into(),
        })
        .await
        .expect("device should be removable");

    let history_access = service
        .history_access("acct-3", "device-1")
        .await
        .expect("history access should resolve after removal");

    assert!(!history_access.allowed);
    assert_eq!(history_access.reason, "device_not_authorized");
}
