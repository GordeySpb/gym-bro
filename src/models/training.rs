use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrainingSession {
    pub id: Uuid,
    pub date: NaiveDate,
    pub exercises: Vec<String>,
    pub duration_minutes: Option<i32>,
    pub notes: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTrainingRequest {
    pub date: NaiveDate,
    pub exercises: Vec<String>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTrainingRequest {
    pub date: Option<NaiveDate>,
    pub exercises: Option<Vec<String>>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}
