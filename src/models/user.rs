use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub id: Uuid,
  pub user_name: String,
  pub email: String,
  pub password_hash: String,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
  pub sub: String,
  pub iat: usize,
  pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
  pub name: String,
  pub email: String,
  pub id: Uuid,
}

// response
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
  pub id: String,
  pub name: String,
  pub email: String,
  pub role: String,
  pub photo: String,
  pub verified: bool,
  pub createdAt: DateTime<Utc>,
  pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
  pub user: FilteredUser,
}
