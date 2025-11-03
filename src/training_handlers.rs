use crate::database::DbPool;
use crate::training_models::{ApiResponse, CreateTrainingRequest, TrainingSession};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn create_training(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTrainingRequest>,
) -> Result<Json<ApiResponse<TrainingSession>>, StatusCode> {
    let training = sqlx::query_as::<_, TrainingSession>(
        r#"
        INSERT INTO training_sessions (date, exercises, duration_minutes, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(payload.date)
    .bind(payload.exercises)
    .bind(payload.duration_minutes)
    .bind(payload.notes.unwrap_or_default())
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: training,
    }))
}
