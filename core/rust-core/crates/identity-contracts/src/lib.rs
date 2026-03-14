use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(value: impl Into<String>) -> Result<Self, AccountIdentityError> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err(AccountIdentityError::InvalidUsername);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Login(String);

impl Login {
    pub fn new(value: impl Into<String>) -> Result<Self, AccountIdentityError> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err(AccountIdentityError::InvalidLogin);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: impl Into<String>) -> Result<Self, AccountIdentityError> {
        let value = value.into().trim().to_string();
        let is_valid =
            value.starts_with('+') && value.len() > 1 && value[1..].chars().all(|ch| ch.is_ascii_digit());
        if !is_valid {
            return Err(AccountIdentityError::InvalidPhoneNumber);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignInId {
    Login(Login),
    PhoneNumber(PhoneNumber),
}

impl SignInId {
    fn as_str(&self) -> &str {
        match self {
            Self::Login(login) => login.as_str(),
            Self::PhoneNumber(phone_number) => phone_number.as_str(),
        }
    }
}

impl From<Login> for SignInId {
    fn from(value: Login) -> Self {
        Self::Login(value)
    }
}

impl From<PhoneNumber> for SignInId {
    fn from(value: PhoneNumber) -> Self {
        Self::PhoneNumber(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountIdentity {
    pub username: Username,
    pub sign_in_ids: Vec<SignInId>,
}

impl AccountIdentity {
    pub fn new(
        username: Username,
        sign_in_ids: Vec<SignInId>,
    ) -> Result<Self, AccountIdentityError> {
        if sign_in_ids.is_empty() {
            return Err(AccountIdentityError::MissingSignInId);
        }

        let mut seen = vec![username.as_str().to_string()];

        for handle in &sign_in_ids {
            let value = handle.as_str().to_string();
            if seen.iter().any(|existing| existing == &value) {
                return Err(AccountIdentityError::DuplicateIdentityValue(value));
            }
            seen.push(value);
        }

        Ok(Self {
            username,
            sign_in_ids,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountIdentityError {
    DuplicateIdentityValue(String),
    InvalidLogin,
    InvalidPhoneNumber,
    InvalidUsername,
    MissingSignInId,
}
