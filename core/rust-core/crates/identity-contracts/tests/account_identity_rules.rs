use identity_contracts::{
    AccountIdentity, AccountIdentityError, Login, PhoneNumber, SignInId, Username,
};

#[test]
fn account_identity_rejects_duplicate_login_username_or_phone() {
    let duplicate_username = AccountIdentity::new(
        Username::new("serenity").unwrap(),
        vec![
            Login::new("serenity").unwrap().into(),
            PhoneNumber::new("+15557654321").unwrap().into(),
        ],
    )
    .unwrap_err();

    assert_eq!(
        duplicate_username,
        AccountIdentityError::DuplicateIdentityValue("serenity".to_string())
    );

    let duplicate_phone = AccountIdentity::new(
        Username::new("another").unwrap(),
        vec![
            Login::new("another-login").unwrap().into(),
            PhoneNumber::new("+15550000000").unwrap().into(),
            PhoneNumber::new("+15550000000").unwrap().into(),
        ],
    )
    .unwrap_err();

    assert_eq!(
        duplicate_phone,
        AccountIdentityError::DuplicateIdentityValue("+15550000000".to_string())
    );
}

#[test]
fn account_identity_contracts_round_trip_through_serde() {
    let identity = AccountIdentity::new(
        Username::new("serenity").unwrap(),
        vec![
            SignInId::from(Login::new("serenity-login").unwrap()),
            SignInId::from(PhoneNumber::new("+15557654321").unwrap()),
        ],
    )
    .unwrap();

    let json = serde_json::to_string(&identity).unwrap();
    let restored: AccountIdentity = serde_json::from_str(&json).unwrap();

    assert_eq!(restored, identity);
}
