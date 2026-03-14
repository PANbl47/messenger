use messenger_gateway::test_support;
use serde_json::json;

#[tokio::test]
async fn add_device_approval_flow_requires_existing_trusted_device_approval() {
    let first = test_support::json(
        "POST",
        "/v1/devices/add-device",
        json!({
            "account_id": "acct-routes-1",
            "device_id": "device-1",
            "device_name": "Primary Phone",
            "device_type": "ios",
            "wrapped_account_key_ref": "wrap-1",
            "last_active_at": 100
        }),
    )
    .await;

    assert_eq!(first.status, 201);
    assert_eq!(first.body["trust_state"], "trusted");

    let blocked = test_support::json(
        "POST",
        "/v1/devices/add-device",
        json!({
            "account_id": "acct-routes-1",
            "device_id": "device-2",
            "device_name": "Tablet",
            "device_type": "ipad",
            "wrapped_account_key_ref": "wrap-2",
            "last_active_at": 200
        }),
    )
    .await;

    assert_eq!(blocked.status, 409);
    assert_eq!(blocked.body["error_code"], "approval_required");

    let approved = test_support::json(
        "POST",
        "/v1/devices/add-device",
        json!({
            "account_id": "acct-routes-1",
            "device_id": "device-2",
            "device_name": "Tablet",
            "device_type": "ipad",
            "wrapped_account_key_ref": "wrap-2",
            "approving_device_id": "device-1",
            "last_active_at": 200
        }),
    )
    .await;

    assert_eq!(approved.status, 201);
    assert_eq!(approved.body["trust_state"], "trusted");
}

#[tokio::test]
async fn remove_device_flow_revokes_the_device() {
    let _ = test_support::json(
        "POST",
        "/v1/devices/add-device",
        json!({
            "account_id": "acct-routes-2",
            "device_id": "device-1",
            "device_name": "Desktop",
            "device_type": "desktop",
            "wrapped_account_key_ref": "wrap-1",
            "last_active_at": 300
        }),
    )
    .await;

    let removed = test_support::json(
        "POST",
        "/v1/devices/remove-device",
        json!({
            "account_id": "acct-routes-2",
            "device_id": "device-1"
        }),
    )
    .await;

    assert_eq!(removed.status, 200);

    let listed = test_support::json("GET", "/v1/devices/list-devices?account_id=acct-routes-2", serde_json::Value::Null).await;

    assert_eq!(listed.status, 200);
    assert_eq!(listed.body["devices"][0]["trust_state"], "revoked");
}

#[tokio::test]
async fn list_devices_includes_last_activity() {
    let _ = test_support::json(
        "POST",
        "/v1/devices/add-device",
        json!({
            "account_id": "acct-routes-3",
            "device_id": "device-1",
            "device_name": "Laptop",
            "device_type": "desktop",
            "wrapped_account_key_ref": "wrap-1",
            "last_active_at": 444
        }),
    )
    .await;

    let listed = test_support::json("GET", "/v1/devices/list-devices?account_id=acct-routes-3", serde_json::Value::Null).await;

    assert_eq!(listed.status, 200);
    assert_eq!(listed.body["devices"][0]["device_name"], "Laptop");
    assert_eq!(listed.body["devices"][0]["last_active_at"], 444);
}

#[tokio::test]
async fn no_trusted_device_recovery_enrollment_path_restores_account_access() {
    let recovered = test_support::json(
        "POST",
        "/v1/devices/recovery-enrollment",
        json!({
            "account_id": "acct-routes-4",
            "device_id": "device-recovery",
            "recovery_wrap_ref": "recovery-wrap-1",
            "last_active_at": 555
        }),
    )
    .await;

    assert_eq!(recovered.status, 200);
    assert_eq!(recovered.body["device"]["device_id"], "device-recovery");
    assert_eq!(recovered.body["history_access_guaranteed"], false);
}
