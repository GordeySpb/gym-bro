mod database;
mod routes;
mod models;
mod app_config;
mod training_session;

use routes::create_router;

const SERVER_ADDRESS: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("failed to load .env file");
    let config = app_config::AppConfig::from_env()?;

    let pool = database::create_poll(config.db_url()).await?;
    println!("✅ Connected to DB");

    let app = create_router(pool);

    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    println!("🚀 Server is started on http://{}", config.server_port());

    Ok(())
}
