use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_config, models::user::User};

pub struct TokenManager;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: Uuid, // user_id
  email: String,
  user_name: String,
}

impl TokenManager {
  pub fn generate_token(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let config = app_config::AppConfig::from_env().unwrap();
    let jwt_sercret = config.jwt_secret();

    let claims = Claims {
      sub: user.id,
      email: user.email.clone(),
      user_name: user.user_name.clone(),
    };

    encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(jwt_sercret),
    )
  }
}
