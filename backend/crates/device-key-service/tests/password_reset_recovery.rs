use device_key_service::{AddDeviceRequest, DeviceKeyService, RecoveryEnrollmentRequest};
use device_trust::{PasswordRecoveryStatus, WrappedKeyRef};

#[tokio::test]
async fn no_trusted_device_recovery_restores_account_access_but_not_guaranteed_history_access() {
    let service = DeviceKeyService::new();

    let recovered = service
        .recover_without_trusted_device(RecoveryEnrollmentRequest {
            account_id: "acct-recovery".into(),
            device_id: "device-recovered".into(),
            recovery_wrap_ref: Some("recovery-wrap-1".into()),
            last_active_at: 500,
        })
        .await
        .expect("recovery should restore account access");

    assert_eq!(recovered.device.device_id, "device-recovered");
    assert!(!recovered.history_access_guaranteed);
    assert_eq!(recovered.device.has_wrapped_account_material, false);
}

#[tokio::test]
async fn password_reset_invalidates_password_derived_recovery_wraps() {
    let service = DeviceKeyService::new();

    let _ = service
        .add_device(AddDeviceRequest {
            account_id: "acct-reset".into(),
            device_id: "device-1".into(),
            device_name: "Primary".into(),
            device_type: "ios".into(),
            wrapped_account_key_ref: Some("wrap-1".into()),
            approving_device_id: None,
            last_active_at: 600,
        })
        .await
        .expect("device should enroll");

    service
        .set_password_recovery_material("acct-reset", WrappedKeyRef::new("pw-wrap-1"))
        .await
        .expect("password recovery material should be stored");

    service
        .reset_password("acct-reset")
        .await
        .expect("password reset should invalidate old recovery material");

    let recovered = service
        .recover_without_trusted_device(RecoveryEnrollmentRequest {
            account_id: "acct-reset".into(),
            device_id: "device-2".into(),
            recovery_wrap_ref: Some("pw-wrap-1".into()),
            last_active_at: 601,
        })
        .await
        .expect("recovery path should still return account access");

    assert_eq!(
        recovered.password_recovery_status,
        PasswordRecoveryStatus::Invalidated
    );
    assert!(!recovered.history_access_guaranteed);
}
