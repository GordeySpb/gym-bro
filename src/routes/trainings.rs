use axum::{
    Json, Router,
    extract::{State,Path},
    http::StatusCode, 
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::{
    database::DbPool,
    training_models::{ApiResponse, CreateTrainingRequest, TrainingSession, UpdateTrainingRequest},
};

const TRAININGS_PATH: &str = "/trainings";
const TRAINING_BY_ID_PATH: &str = "/trainings/:id";

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route(TRAININGS_PATH, get(get_all))
        .route(TRAININGS_PATH, post(create))
        .route(TRAINING_BY_ID_PATH, put(update))
}

pub async fn get_all(
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

pub async fn create(
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

pub async fn update(
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
