use account_service::{
    AccountService, EmailLink, LinkLoginPassword, LinkPhone, LoginFirstRegistration,
    PhoneFirstRegistration,
};
use persistence::InMemoryStore;

#[tokio::test]
async fn phone_first_registration_requires_verified_phone_username_and_display_name() {
    let store = InMemoryStore::default();
    let service = AccountService::new(store);

    let created = service
        .register_phone_first(PhoneFirstRegistration {
            phone: "+15550000001".into(),
            username: "serenity".into(),
            display_name: "Serenity".into(),
            phone_discoverable: true,
        })
        .await
        .expect("phone-first registration should succeed");

    assert_eq!(created.username, "serenity");
    assert_eq!(created.display_name, "Serenity");
    assert_eq!(created.phone.as_deref(), Some("+15550000001"));
    assert_eq!(created.login.as_deref(), None);

    let duplicate_phone = service
        .register_phone_first(PhoneFirstRegistration {
            phone: "+15550000001".into(),
            username: "serenity-two".into(),
            display_name: "Serenity Two".into(),
            phone_discoverable: false,
        })
        .await
        .unwrap_err();

    assert_eq!(duplicate_phone.code(), "duplicate_phone");
}

#[tokio::test]
async fn login_first_registration_requires_unique_login_password_username_and_display_name() {
    let store = InMemoryStore::default();
    let service = AccountService::new(store);

    let created = service
        .register_login_first(LoginFirstRegistration {
            login: "serenity-login".into(),
            password: "correct horse battery staple".into(),
            username: "serenity".into(),
            display_name: "Serenity".into(),
        })
        .await
        .expect("login-first registration should succeed");

    assert_eq!(created.login.as_deref(), Some("serenity-login"));
    assert_eq!(created.phone.as_deref(), None);

    let duplicate_login = service
        .register_login_first(LoginFirstRegistration {
            login: "serenity-login".into(),
            password: "another password".into(),
            username: "serenity-two".into(),
            display_name: "Serenity Two".into(),
        })
        .await
        .unwrap_err();

    assert_eq!(duplicate_login.code(), "duplicate_login");

    let duplicate_username = service
        .register_login_first(LoginFirstRegistration {
            login: "serenity-login-two".into(),
            password: "another password".into(),
            username: "serenity".into(),
            display_name: "Serenity Three".into(),
        })
        .await
        .unwrap_err();

    assert_eq!(duplicate_username.code(), "duplicate_username");
}

#[tokio::test]
async fn phone_first_account_can_link_login_and_password_later() {
    let store = InMemoryStore::default();
    let service = AccountService::new(store);

    let account = service
        .register_phone_first(PhoneFirstRegistration {
            phone: "+15550000002".into(),
            username: "phone-first".into(),
            display_name: "Phone First".into(),
            phone_discoverable: false,
        })
        .await
        .unwrap();

    let updated = service
        .link_login_password(LinkLoginPassword {
            account_id: account.account_id.clone(),
            login: "phone-first-login".into(),
            password: "linked password".into(),
        })
        .await
        .expect("linking login/password should succeed");

    assert_eq!(updated.login.as_deref(), Some("phone-first-login"));
    assert_eq!(updated.phone.as_deref(), Some("+15550000002"));
}

#[tokio::test]
async fn login_first_account_can_link_phone_later() {
    let store = InMemoryStore::default();
    let service = AccountService::new(store);

    let account = service
        .register_login_first(LoginFirstRegistration {
            login: "login-first".into(),
            password: "linked password".into(),
            username: "login-first-user".into(),
            display_name: "Login First".into(),
        })
        .await
        .unwrap();

    let updated = service
        .link_phone(LinkPhone {
            account_id: account.account_id.clone(),
            phone: "+15550000003".into(),
            phone_discoverable: true,
        })
        .await
        .expect("linking phone should succeed");

    assert_eq!(updated.login.as_deref(), Some("login-first"));
    assert_eq!(updated.phone.as_deref(), Some("+15550000003"));
}

#[tokio::test]
async fn link_email_persists_recovery_metadata_without_exposing_private_content() {
    let store = InMemoryStore::default();
    let service = AccountService::new(store.clone());

    let account = service
        .register_login_first(LoginFirstRegistration {
            login: "email-user".into(),
            password: "safe password".into(),
            username: "email-user".into(),
            display_name: "Email User".into(),
        })
        .await
        .unwrap();

    let linked = service
        .link_email(EmailLink {
            account_id: account.account_id.clone(),
            email: "owner@example.com".into(),
        })
        .await
        .expect("linking recovery email should succeed");

    assert_eq!(linked.recovery_email_hint, "o***@example.com");

    let stored = store
        .account_by_id(&account.account_id)
        .expect("account should exist after link");
    let recovery = stored
        .recovery_email
        .expect("recovery email should be persisted privately");

    assert_eq!(recovery.email, "owner@example.com");
    assert_eq!(recovery.hint, "o***@example.com");
}
