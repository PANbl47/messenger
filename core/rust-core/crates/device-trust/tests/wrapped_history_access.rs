use device_trust::{DeviceRecord, DeviceTrustState, HistoryAccess};

#[test]
fn history_access_requires_authorized_device_plus_wrapped_account_material() {
    let trusted_without_wrap = DeviceRecord::new(
        "device-1",
        "Primary",
        "ios",
        DeviceTrustState::Trusted,
        false,
        100,
    );
    let trusted_with_wrap = DeviceRecord::new(
        "device-2",
        "Tablet",
        "ipad",
        DeviceTrustState::Trusted,
        true,
        200,
    );
    let recovery_only = DeviceRecord::new(
        "device-3",
        "Recovered",
        "android",
        DeviceTrustState::RecoveryOnly,
        false,
        300,
    );

    assert_eq!(
        HistoryAccess::from_device(&trusted_without_wrap),
        HistoryAccess::denied("missing_wrapped_account_material")
    );
    assert_eq!(
        HistoryAccess::from_device(&recovery_only),
        HistoryAccess::denied("device_not_authorized")
    );
    assert_eq!(
        HistoryAccess::from_device(&trusted_with_wrap),
        HistoryAccess::allowed()
    );
}
