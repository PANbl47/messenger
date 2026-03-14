use messenger_gateway::test_support;
use serde_json::{json, Value};

#[tokio::test]
async fn phone_first_signup_route_returns_created_account() {
    let response = test_support::json(
        "POST",
        "/v1/accounts/sign-up/phone",
        json!({
            "phone": "+15550001000",
            "username": "serenity",
            "display_name": "Serenity",
            "phone_discoverable": true
        }),
    )
    .await;

    assert_eq!(response.status, 201);
    assert_eq!(response.body["username"], "serenity");
    assert_eq!(response.body["phone"], "+15550001000");
}

#[tokio::test]
async fn login_first_signup_route_returns_created_account() {
    let response = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "serenity-login",
            "password": "safe password",
            "username": "serenity-login-user",
            "display_name": "Serenity Login"
        }),
    )
    .await;

    assert_eq!(response.status, 201);
    assert_eq!(response.body["login"], "serenity-login");
    assert_eq!(response.body["username"], "serenity-login-user");
}

#[tokio::test]
async fn link_email_route_returns_masked_recovery_hint() {
    let account = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "recovery-login",
            "password": "safe password",
            "username": "recovery-user",
            "display_name": "Recovery User"
        }),
    )
    .await;

    let response = test_support::json(
        "POST",
        "/v1/accounts/link-email",
        json!({
            "account_id": account.body["account_id"],
            "email": "owner@example.com"
        }),
    )
    .await;

    assert_eq!(response.status, 200);
    assert_eq!(response.body["recovery_email_hint"], "o***@example.com");
    assert_eq!(response.body.get("email"), None);
}

#[tokio::test]
async fn linking_phone_or_login_later_updates_the_account() {
    let phone_first = test_support::json(
        "POST",
        "/v1/accounts/sign-up/phone",
        json!({
            "phone": "+15550001001",
            "username": "phone-first",
            "display_name": "Phone First",
            "phone_discoverable": false
        }),
    )
    .await;

    let login_link = test_support::json(
        "POST",
        "/v1/accounts/link-login-password",
        json!({
            "account_id": phone_first.body["account_id"],
            "login": "phone-first-login",
            "password": "linked password"
        }),
    )
    .await;

    assert_eq!(login_link.status, 200);
    assert_eq!(login_link.body["login"], "phone-first-login");

    let login_first = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "login-first",
            "password": "safe password",
            "username": "login-first-user",
            "display_name": "Login First"
        }),
    )
    .await;

    let phone_link = test_support::json(
        "POST",
        "/v1/accounts/link-phone",
        json!({
            "account_id": login_first.body["account_id"],
            "phone": "+15550001002",
            "phone_discoverable": true
        }),
    )
    .await;

    assert_eq!(phone_link.status, 200);
    assert_eq!(phone_link.body["phone"], "+15550001002");
}

#[tokio::test]
async fn username_and_display_name_search_routes_return_expected_results() {
    let _ = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "search-a",
            "password": "safe password",
            "username": "sam-exact",
            "display_name": "Sam"
        }),
    )
    .await;

    let _ = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "search-b",
            "password": "safe password",
            "username": "sam-rivera",
            "display_name": "Sam Rivera"
        }),
    )
    .await;

    let username = test_support::json("GET", "/v1/directory/username/lookup?value=sam-exact", Value::Null).await;
    assert_eq!(username.status, 200);
    assert_eq!(username.body["username"], "sam-exact");

    let display_name = test_support::json("GET", "/v1/directory/display-name/search?value=Sam", Value::Null).await;
    assert_eq!(display_name.status, 200);
    assert_eq!(display_name.body["results"][0]["display_name"], "Sam");
    assert_eq!(display_name.body["results"][1]["display_name"], "Sam Rivera");
}

#[tokio::test]
async fn phone_search_route_obeys_privacy_control() {
    let _ = test_support::json(
        "POST",
        "/v1/accounts/sign-up/phone",
        json!({
            "phone": "+15550001003",
            "username": "discoverable",
            "display_name": "Discoverable",
            "phone_discoverable": true
        }),
    )
    .await;

    let _ = test_support::json(
        "POST",
        "/v1/accounts/sign-up/phone",
        json!({
            "phone": "+15550001004",
            "username": "hidden",
            "display_name": "Hidden",
            "phone_discoverable": false
        }),
    )
    .await;

    let discoverable = test_support::json("GET", "/v1/directory/phone/lookup?value=%2B15550001003", Value::Null).await;
    let hidden = test_support::json("GET", "/v1/directory/phone/lookup?value=%2B15550001004", Value::Null).await;

    assert_eq!(discoverable.status, 200);
    assert_eq!(discoverable.body["username"], "discoverable");
    assert_eq!(hidden.status, 404);
}

#[tokio::test]
async fn duplicate_identity_values_return_stable_error_codes() {
    let first = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "duplicate-login",
            "password": "safe password",
            "username": "duplicate-user",
            "display_name": "Duplicate User"
        }),
    )
    .await;

    assert_eq!(first.status, 201);

    let duplicate_login = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "duplicate-login",
            "password": "safe password",
            "username": "duplicate-user-2",
            "display_name": "Duplicate User Two"
        }),
    )
    .await;
    assert_eq!(duplicate_login.status, 409);
    assert_eq!(duplicate_login.body["error_code"], "duplicate_login");

    let duplicate_username = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "duplicate-login-2",
            "password": "safe password",
            "username": "duplicate-user",
            "display_name": "Duplicate User Three"
        }),
    )
    .await;
    assert_eq!(duplicate_username.status, 409);
    assert_eq!(duplicate_username.body["error_code"], "duplicate_username");

    let account = test_support::json(
        "POST",
        "/v1/accounts/sign-up/login",
        json!({
            "login": "phone-link-owner",
            "password": "safe password",
            "username": "phone-link-owner",
            "display_name": "Phone Link Owner"
        }),
    )
    .await;

    let _ = test_support::json(
        "POST",
        "/v1/accounts/sign-up/phone",
        json!({
            "phone": "+15550001005",
            "username": "phone-user",
            "display_name": "Phone User",
            "phone_discoverable": true
        }),
    )
    .await;

    let duplicate_phone = test_support::json(
        "POST",
        "/v1/accounts/link-phone",
        json!({
            "account_id": account.body["account_id"],
            "phone": "+15550001005",
            "phone_discoverable": true
        }),
    )
    .await;

    assert_eq!(duplicate_phone.status, 409);
    assert_eq!(duplicate_phone.body["error_code"], "duplicate_phone");
}
