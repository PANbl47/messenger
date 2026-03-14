use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneFirstRegistration {
    pub phone: String,
    pub username: String,
    pub display_name: String,
    pub phone_discoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginFirstRegistration {
    pub login: String,
    pub password: String,
    pub username: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkLoginPassword {
    pub account_id: String,
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkPhone {
    pub account_id: String,
    pub phone: String,
    pub phone_discoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailLink {
    pub account_id: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountRecord {
    pub account_id: String,
    pub username: String,
    pub display_name: String,
    pub login: Option<String>,
    pub phone: Option<String>,
    pub phone_discoverable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryEmailView {
    pub account_id: String,
    pub recovery_email_hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignInBootstrap {
    pub account_id: String,
    pub username: String,
    pub requires_password: bool,
    pub sign_in_method: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountError {
    DuplicateLogin,
    DuplicatePhone,
    DuplicateUsername,
    AccountNotFound,
    InvalidDisplayName,
    InvalidEmail,
    InvalidLogin,
    InvalidPassword,
    InvalidPhone,
    InvalidUsername,
    LoginAlreadyLinked,
    PhoneAlreadyLinked,
    RecoveryEmailAlreadyLinked,
    SignInIdNotFound,
}

impl AccountError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::DuplicateLogin => "duplicate_login",
            Self::DuplicatePhone => "duplicate_phone",
            Self::DuplicateUsername => "duplicate_username",
            Self::AccountNotFound => "account_not_found",
            Self::InvalidDisplayName => "invalid_display_name",
            Self::InvalidEmail => "invalid_email",
            Self::InvalidLogin => "invalid_login",
            Self::InvalidPassword => "invalid_password",
            Self::InvalidPhone => "invalid_phone",
            Self::InvalidUsername => "invalid_username",
            Self::LoginAlreadyLinked => "login_already_linked",
            Self::PhoneAlreadyLinked => "phone_already_linked",
            Self::RecoveryEmailAlreadyLinked => "recovery_email_already_linked",
            Self::SignInIdNotFound => "sign_in_id_not_found",
        }
    }
}
