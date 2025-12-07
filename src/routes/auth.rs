use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  routing::{delete, get, post, put},
};

use crate::{
  database::DbPool,
  models::{
    response::ApiResponse,
    user::{LoginUserSchema, RegisterUserRequest, User, UserResponse},
  },
  utils::password_manager::PasswordManager,
};

const REGISTER_PATH: &str = "/auth/register";
const LOGIN_PATH: &str = "/auth/login";

const MIN_EMAIL_LEN: usize = 5;

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

  if !payload.email.contains('@') || payload.email.len() < MIN_EMAIL_LEN {
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

pub async fn login(
  State(pool): State<DbPool>,
  Json(payload): Json<LoginUserSchema>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
  let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  let is_valid_password = PasswordManager::verify_password(&payload.password, &user.password_hash)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  if !is_valid_password {
    return Err(StatusCode::UNAUTHORIZED);
  }

  Ok(Json(ApiResponse {
    success: true,
    data: (),
  }))
}

pub fn routes() -> Router<DbPool> {
  Router::new().route(REGISTER_PATH, post(register)).route(LOGIN_PATH, post(login))
}
