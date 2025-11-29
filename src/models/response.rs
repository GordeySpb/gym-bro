use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
  pub success: bool,
  pub data: T,
}
