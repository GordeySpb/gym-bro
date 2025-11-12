mod app_config;
mod database;
mod models;
mod routes;
mod training_session;

use routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("failed to load .env file");
    let config = app_config::AppConfig::from_env()?;

    let pool = database::create_poll(config.db_url()).await?;
    println!("✅ Connected to DB");

    let app = create_router(pool);

    let listener = tokio::net::TcpListener::bind(config.full_server_address()).await.unwrap();

    println!("🚀 Server is started on http://{}", config.server_port());

    axum::serve(listener, app).await.unwrap();

    println!("Server stopped");

    Ok(())
}
