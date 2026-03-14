use device_trust::{DeviceEnrollment, DeviceId, DeviceTrustState, WrappedKeyRef};

#[test]
fn new_signed_in_device_is_untrusted_until_enrolled() {
    let enrollment = DeviceEnrollment::signed_in(
        DeviceId::new("device-1"),
        WrappedKeyRef::new("wrapped-key-1"),
    );

    assert_eq!(enrollment.trust_state(), DeviceTrustState::Untrusted);

    let trusted = enrollment.enroll();
    assert_eq!(trusted.trust_state(), DeviceTrustState::Trusted);
}

#[test]
fn device_trust_contracts_round_trip_through_serde() {
    let enrollment = DeviceEnrollment::signed_in(
        DeviceId::new("device-serde"),
        WrappedKeyRef::new("wrapped-key-serde"),
    )
    .enroll();

    let json = serde_json::to_string(&enrollment).unwrap();
    let restored: DeviceEnrollment = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, enrollment);
}
