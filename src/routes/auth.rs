use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{delete, get, post, put},
};
use bcrypt::{DEFAULT_COST, hash, verify};

use crate::{
  database::DbPool,
  models::{
    response::ApiResponse,
    user::{RegisterUserRequest, UserResponse},
  },
  utils::password_manager::PasswordManager,
};

const REGISTER_PATH: &str = "/auth/register";
const LOGIN_PATH: &str = "/auth/login";

pub async fn register(
  State(pool): State<DbPool>,
  Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
  let is_user_exist = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  if is_user_exist {
    return Err(StatusCode::CONFLICT);
  }

  if !payload.email.contains('@') || payload.email.len() < 5 {
    eprintln!("email dosn't valid");
    return Err(StatusCode::CONFLICT);
  }

  if !PasswordManager::is_strong_password(&payload.password) {
    eprintln!("password not strong enaugh");
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  }

  let password_hash = PasswordManager::hash_password(&payload.password).map_err(|e| {
    eprintln!("Hashing password was wrong: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  let user_id = sqlx::query_scalar(
    "INSERT INTO users (user_name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
  )
  .bind(&payload.name)
  .bind(&payload.email)
  .bind(&password_hash)
  .fetch_one(&pool)
  .await
  .map_err(|e| {
    eprintln!("Saving user was failed: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  let user_data = UserResponse {
    name: payload.name,
    email: payload.email,
    id: user_id,
  };

  Ok(Json(ApiResponse {
    success: true,
    data: user_data,
  }))
}

pub fn routes() -> Router<DbPool> {
  Router::new().route(REGISTER_PATH, post(register))
}
