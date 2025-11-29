use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
pub struct PasswordManager;

impl PasswordManager {
  pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
  }

  pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, BcryptError> {
    verify(password, password_hash)
  }

  pub fn is_strong_password(password: &str) -> bool {
    password.len() >= 8
      && password.chars().any(|c| c.is_ascii_uppercase())
      && password.chars().any(|c| c.is_ascii_lowercase())
      && password.chars().any(|c| c.is_ascii_digit())
  }
}
