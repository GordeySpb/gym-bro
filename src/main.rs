use axum::{
    Router,
    routing::{get, post, put},
};

mod database;
mod training_handlers;
mod training_models;
mod training_session;

const SERVER_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = database::create_poll().await?;
    println!("✅ Connected to DB");

    let app = Router::new()
        .route("/", get(|| async { "Gym Bro!" }))
        .route("/trainings", post(training_handlers::create_training))
        .route("/trainings/:id", put(training_handlers::update_training))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    println!("🚀 Server is started on http://{}", SERVER_ADDRESS);

    Ok(())
}
