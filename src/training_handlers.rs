use crate::database::DbPool;
use crate::training_models::{
    ApiResponse, CreateTrainingRequest, TrainingSession, UpdateTrainingRequest,
};
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

pub async fn update_training(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTrainingRequest>,
) -> Result<Json<ApiResponse<TrainingSession>>, StatusCode> {
    let training = sqlx::query_as::<_, TrainingSession>(
        r#"
        UPDATE training_sessions 
        SET 
            date = COALESCE($1, date),
            exercises = COALESCE($2, exercises),
            duration_minutes = COALESCE($3, duration_minutes),
            notes = COALESCE($4, notes)
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(payload.date)
    .bind(payload.exercises)
    .bind(payload.duration_minutes)
    .bind(payload.notes)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Ошибка обновления: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match training {
        Some(training) => Ok(Json(ApiResponse {
            success: true,
            data: training,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_all_trainings(
    State(pool): State<DbPool>,
) -> Result<Json<ApiResponse<Vec<TrainingSession>>>, StatusCode> {
    let trainings =
        sqlx::query_as::<_, TrainingSession>("SELECT * FROM training_sessions ORDER BY date ASC")
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: trainings,
    }))
}
