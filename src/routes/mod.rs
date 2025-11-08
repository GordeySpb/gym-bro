pub mod trainings;
use crate::database::DbPool;
use axum::Router;

pub fn create_router(pool: DbPool) -> Router {
    Router::new().merge(trainings::routes()).with_state(pool)
}
