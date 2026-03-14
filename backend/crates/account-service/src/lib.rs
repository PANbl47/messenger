mod model;
mod repository;

use identity_contracts::{Login, PhoneNumber, Username};
use persistence::{InMemoryStore, StoreError, StoredAccount};

pub use model::{
    AccountError, AccountRecord, EmailLink, LinkLoginPassword, LinkPhone,
    LoginFirstRegistration, PhoneFirstRegistration, RecoveryEmailView, SignInBootstrap,
};
use repository::AccountRepository;

#[derive(Debug, Clone)]
pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    pub fn new(store: InMemoryStore) -> Self {
        Self {
            repository: AccountRepository::new(store),
        }
    }

    pub async fn register_phone_first(
        &self,
        registration: PhoneFirstRegistration,
    ) -> Result<AccountRecord, AccountError> {
        validate_username(&registration.username)?;
        validate_display_name(&registration.display_name)?;
        validate_phone(&registration.phone)?;

        self.repository
            .create_phone_first(
                registration.username,
                registration.display_name,
                registration.phone,
                registration.phone_discoverable,
            )
            .map(map_account)
            .map_err(map_store_error)
    }

    pub async fn register_login_first(
        &self,
        registration: LoginFirstRegistration,
    ) -> Result<AccountRecord, AccountError> {
        validate_username(&registration.username)?;
        validate_display_name(&registration.display_name)?;
        validate_login(&registration.login)?;
        validate_password(&registration.password)?;

        self.repository
            .create_login_first(
                registration.username,
                registration.display_name,
                registration.login,
                registration.password,
            )
            .map(map_account)
            .map_err(map_store_error)
    }

    pub async fn link_login_password(
        &self,
        link: LinkLoginPassword,
    ) -> Result<AccountRecord, AccountError> {
        validate_login(&link.login)?;
        validate_password(&link.password)?;

        self.repository
            .link_login_password(&link.account_id, link.login, link.password)
            .map(map_account)
            .map_err(map_store_error)
    }

    pub async fn link_phone(&self, link: LinkPhone) -> Result<AccountRecord, AccountError> {
        validate_phone(&link.phone)?;

        self.repository
            .link_phone(&link.account_id, link.phone, link.phone_discoverable)
            .map(map_account)
            .map_err(map_store_error)
    }

    pub async fn link_email(&self, link: EmailLink) -> Result<RecoveryEmailView, AccountError> {
        validate_email(&link.email)?;
        let hint = mask_email(&link.email)?;

        self.repository
            .link_email(&link.account_id, link.email, hint.clone())
            .map_err(map_store_error)?;

        Ok(RecoveryEmailView {
            account_id: link.account_id,
            recovery_email_hint: hint,
        })
    }

    pub async fn sign_in_bootstrap(
        &self,
        sign_in_id: &str,
    ) -> Result<SignInBootstrap, AccountError> {
        let account = if sign_in_id.starts_with('+') {
            validate_phone(sign_in_id)?;
            self.repository.get_by_phone(sign_in_id)
        } else {
            validate_login(sign_in_id)?;
            self.repository.get_by_login(sign_in_id)
        }
        .ok_or(AccountError::SignInIdNotFound)?;

        Ok(SignInBootstrap {
            account_id: account.account_id,
            username: account.username,
            requires_password: account.password.is_some(),
            sign_in_method: if sign_in_id.starts_with('+') {
                "phone".to_string()
            } else {
                "login".to_string()
            },
        })
    }
}

fn validate_username(value: &str) -> Result<(), AccountError> {
    Username::new(value).map_err(|_| AccountError::InvalidUsername)?;
    Ok(())
}

fn validate_login(value: &str) -> Result<(), AccountError> {
    Login::new(value).map_err(|_| AccountError::InvalidLogin)?;
    Ok(())
}

fn validate_phone(value: &str) -> Result<(), AccountError> {
    PhoneNumber::new(value).map_err(|_| AccountError::InvalidPhone)?;
    Ok(())
}

fn validate_display_name(value: &str) -> Result<(), AccountError> {
    if value.trim().is_empty() {
        return Err(AccountError::InvalidDisplayName);
    }
    Ok(())
}

fn validate_password(value: &str) -> Result<(), AccountError> {
    if value.trim().is_empty() {
        return Err(AccountError::InvalidPassword);
    }
    Ok(())
}

fn validate_email(value: &str) -> Result<(), AccountError> {
    let trimmed = value.trim();
    if trimmed.is_empty() || !trimmed.contains('@') {
        return Err(AccountError::InvalidEmail);
    }
    Ok(())
}

fn mask_email(value: &str) -> Result<String, AccountError> {
    let (local, domain) = value.split_once('@').ok_or(AccountError::InvalidEmail)?;
    let first = local.chars().next().ok_or(AccountError::InvalidEmail)?;
    Ok(format!("{first}***@{domain}"))
}

fn map_account(account: StoredAccount) -> AccountRecord {
    AccountRecord {
        account_id: account.account_id,
        username: account.username,
        display_name: account.display_name,
        login: account.login,
        phone: account.phone,
        phone_discoverable: account.phone_discoverable,
    }
}

fn map_store_error(error: StoreError) -> AccountError {
    match error {
        StoreError::DuplicateLogin => AccountError::DuplicateLogin,
        StoreError::DuplicatePhone => AccountError::DuplicatePhone,
        StoreError::DuplicateUsername => AccountError::DuplicateUsername,
        StoreError::AccountNotFound => AccountError::AccountNotFound,
        StoreError::LoginAlreadyLinked => AccountError::LoginAlreadyLinked,
        StoreError::PhoneAlreadyLinked => AccountError::PhoneAlreadyLinked,
        StoreError::RecoveryEmailAlreadyLinked => AccountError::RecoveryEmailAlreadyLinked,
    }
}
