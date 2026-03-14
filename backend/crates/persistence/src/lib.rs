use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEmail {
    pub email: String,
    pub hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAccount {
    pub account_id: String,
    pub username: String,
    pub display_name: String,
    pub login: Option<String>,
    pub password: Option<String>,
    pub phone: Option<String>,
    pub phone_discoverable: bool,
    pub recovery_email: Option<RecoveryEmail>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    DuplicateLogin,
    DuplicatePhone,
    DuplicateUsername,
    AccountNotFound,
    LoginAlreadyLinked,
    PhoneAlreadyLinked,
    RecoveryEmailAlreadyLinked,
}

#[derive(Debug, Default)]
struct StoreState {
    next_id: u64,
    accounts: HashMap<String, StoredAccount>,
    login_index: HashMap<String, String>,
    phone_index: HashMap<String, String>,
    username_index: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryStore {
    inner: Arc<Mutex<StoreState>>,
}

impl InMemoryStore {
    pub fn create_account(
        &self,
        username: String,
        display_name: String,
        login: Option<String>,
        password: Option<String>,
        phone: Option<String>,
        phone_discoverable: bool,
    ) -> Result<StoredAccount, StoreError> {
        let mut state = self.inner.lock().expect("store lock should not be poisoned");

        if state.username_index.contains_key(&username) {
            return Err(StoreError::DuplicateUsername);
        }
        if let Some(login_value) = &login {
            if state.login_index.contains_key(login_value) {
                return Err(StoreError::DuplicateLogin);
            }
        }
        if let Some(phone_value) = &phone {
            if state.phone_index.contains_key(phone_value) {
                return Err(StoreError::DuplicatePhone);
            }
        }

        state.next_id += 1;
        let account_id = format!("acct-{}", state.next_id);
        let account = StoredAccount {
            account_id: account_id.clone(),
            username: username.clone(),
            display_name,
            login: login.clone(),
            password,
            phone: phone.clone(),
            phone_discoverable,
            recovery_email: None,
        };

        state.username_index.insert(username, account_id.clone());
        if let Some(login_value) = login {
            state.login_index.insert(login_value, account_id.clone());
        }
        if let Some(phone_value) = phone {
            state.phone_index.insert(phone_value, account_id.clone());
        }
        state.accounts.insert(account_id, account.clone());

        Ok(account)
    }

    pub fn link_login_password(
        &self,
        account_id: &str,
        login: String,
        password: String,
    ) -> Result<StoredAccount, StoreError> {
        let mut state = self.inner.lock().expect("store lock should not be poisoned");

        if state.login_index.contains_key(&login) {
            return Err(StoreError::DuplicateLogin);
        }

        let account = state
            .accounts
            .get_mut(account_id)
            .ok_or(StoreError::AccountNotFound)?;

        if account.login.is_some() {
            return Err(StoreError::LoginAlreadyLinked);
        }

        account.login = Some(login.clone());
        account.password = Some(password);
        let updated = account.clone();
        state.login_index.insert(login, account_id.to_string());

        Ok(updated)
    }

    pub fn link_phone(
        &self,
        account_id: &str,
        phone: String,
        phone_discoverable: bool,
    ) -> Result<StoredAccount, StoreError> {
        let mut state = self.inner.lock().expect("store lock should not be poisoned");

        if state.phone_index.contains_key(&phone) {
            return Err(StoreError::DuplicatePhone);
        }

        let account = state
            .accounts
            .get_mut(account_id)
            .ok_or(StoreError::AccountNotFound)?;

        if account.phone.is_some() {
            return Err(StoreError::PhoneAlreadyLinked);
        }

        account.phone = Some(phone.clone());
        account.phone_discoverable = phone_discoverable;
        let updated = account.clone();
        state.phone_index.insert(phone, account_id.to_string());

        Ok(updated)
    }

    pub fn link_recovery_email(
        &self,
        account_id: &str,
        recovery_email: RecoveryEmail,
    ) -> Result<StoredAccount, StoreError> {
        let mut state = self.inner.lock().expect("store lock should not be poisoned");
        let account = state
            .accounts
            .get_mut(account_id)
            .ok_or(StoreError::AccountNotFound)?;

        if account.recovery_email.is_some() {
            return Err(StoreError::RecoveryEmailAlreadyLinked);
        }

        account.recovery_email = Some(recovery_email);
        Ok(account.clone())
    }

    pub fn account_by_id(&self, account_id: &str) -> Option<StoredAccount> {
        self.inner
            .lock()
            .expect("store lock should not be poisoned")
            .accounts
            .get(account_id)
            .cloned()
    }

    pub fn account_by_username(&self, username: &str) -> Option<StoredAccount> {
        let state = self.inner.lock().expect("store lock should not be poisoned");
        let account_id = state.username_index.get(username)?;
        state.accounts.get(account_id).cloned()
    }

    pub fn account_by_login(&self, login: &str) -> Option<StoredAccount> {
        let state = self.inner.lock().expect("store lock should not be poisoned");
        let account_id = state.login_index.get(login)?;
        state.accounts.get(account_id).cloned()
    }

    pub fn account_by_phone(&self, phone: &str) -> Option<StoredAccount> {
        let state = self.inner.lock().expect("store lock should not be poisoned");
        let account_id = state.phone_index.get(phone)?;
        state.accounts.get(account_id).cloned()
    }

    pub fn all_accounts(&self) -> Vec<StoredAccount> {
        self.inner
            .lock()
            .expect("store lock should not be poisoned")
            .accounts
            .values()
            .cloned()
            .collect()
    }
}
