pub mod auth;
pub mod trainings;
use axum::Router;

use crate::database::DbPool;

pub fn create_router(pool: DbPool) -> Router {
  Router::new().merge(trainings::routes()).merge(auth::routes()).with_state(pool)
}
