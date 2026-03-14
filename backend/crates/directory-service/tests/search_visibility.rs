use account_service::{AccountService, LoginFirstRegistration, PhoneFirstRegistration};
use directory_service::DirectoryService;
use persistence::InMemoryStore;

#[tokio::test]
async fn username_search_returns_exact_unique_match() {
    let store = InMemoryStore::default();
    let accounts = AccountService::new(store.clone());
    let directory = DirectoryService::new(store);

    accounts
        .register_login_first(LoginFirstRegistration {
            login: "serenity-login".into(),
            password: "safe password".into(),
            username: "serenity".into(),
            display_name: "Serenity".into(),
        })
        .await
        .unwrap();

    let result = directory
        .search_by_username("serenity")
        .await
        .expect("username should resolve");

    assert_eq!(result.username, "serenity");
    assert_eq!(result.display_name, "Serenity");
}

#[tokio::test]
async fn display_name_search_returns_ranked_disambiguated_results() {
    let store = InMemoryStore::default();
    let accounts = AccountService::new(store.clone());
    let directory = DirectoryService::new(store);

    accounts
        .register_login_first(LoginFirstRegistration {
            login: "sam-exact".into(),
            password: "safe password".into(),
            username: "sam-exact".into(),
            display_name: "Sam".into(),
        })
        .await
        .unwrap();

    accounts
        .register_login_first(LoginFirstRegistration {
            login: "sam-rivera".into(),
            password: "safe password".into(),
            username: "sam-rivera".into(),
            display_name: "Sam Rivera".into(),
        })
        .await
        .unwrap();

    accounts
        .register_login_first(LoginFirstRegistration {
            login: "sam-stone".into(),
            password: "safe password".into(),
            username: "sam-stone".into(),
            display_name: "Samantha Stone".into(),
        })
        .await
        .unwrap();

    let results = directory.search_by_display_name("Sam").await;

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].display_name, "Sam");
    assert_eq!(results[0].disambiguation, "@sam-exact");
    assert_eq!(results[1].display_name, "Sam Rivera");
    assert_eq!(results[2].display_name, "Samantha Stone");
}

#[tokio::test]
async fn phone_search_respects_target_privacy_setting() {
    let store = InMemoryStore::default();
    let accounts = AccountService::new(store.clone());
    let directory = DirectoryService::new(store);

    accounts
        .register_phone_first(PhoneFirstRegistration {
            phone: "+15550000004".into(),
            username: "discoverable".into(),
            display_name: "Discoverable User".into(),
            phone_discoverable: true,
        })
        .await
        .unwrap();

    accounts
        .register_phone_first(PhoneFirstRegistration {
            phone: "+15550000005".into(),
            username: "private".into(),
            display_name: "Private User".into(),
            phone_discoverable: false,
        })
        .await
        .unwrap();

    let discoverable = directory.search_by_phone("+15550000004").await;
    let hidden = directory.search_by_phone("+15550000005").await;

    assert_eq!(discoverable.expect("discoverable user").username, "discoverable");
    assert!(hidden.is_none(), "private phone should not be searchable");
}
