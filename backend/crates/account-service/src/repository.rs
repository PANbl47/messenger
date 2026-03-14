use persistence::{InMemoryStore, RecoveryEmail, StoreError, StoredAccount};

#[derive(Debug, Clone)]
pub struct AccountRepository {
    store: InMemoryStore,
}

impl AccountRepository {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }

    pub fn create_phone_first(
        &self,
        username: String,
        display_name: String,
        phone: String,
        phone_discoverable: bool,
    ) -> Result<StoredAccount, StoreError> {
        self.store.create_account(
            username,
            display_name,
            None,
            None,
            Some(phone),
            phone_discoverable,
        )
    }

    pub fn create_login_first(
        &self,
        username: String,
        display_name: String,
        login: String,
        password: String,
    ) -> Result<StoredAccount, StoreError> {
        self.store.create_account(
            username,
            display_name,
            Some(login),
            Some(password),
            None,
            false,
        )
    }

    pub fn link_login_password(
        &self,
        account_id: &str,
        login: String,
        password: String,
    ) -> Result<StoredAccount, StoreError> {
        self.store.link_login_password(account_id, login, password)
    }

    pub fn link_phone(
        &self,
        account_id: &str,
        phone: String,
        phone_discoverable: bool,
    ) -> Result<StoredAccount, StoreError> {
        self.store.link_phone(account_id, phone, phone_discoverable)
    }

    pub fn link_email(
        &self,
        account_id: &str,
        email: String,
        hint: String,
    ) -> Result<StoredAccount, StoreError> {
        self.store.link_recovery_email(account_id, RecoveryEmail { email, hint })
    }

    pub fn get_by_login(&self, login: &str) -> Option<StoredAccount> {
        self.store.account_by_login(login)
    }

    pub fn get_by_phone(&self, phone: &str) -> Option<StoredAccount> {
        self.store.account_by_phone(phone)
    }
}
