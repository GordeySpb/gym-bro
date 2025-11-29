use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
pub struct PasswordManager;

impl PasswordManager {
  pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
  }
}
