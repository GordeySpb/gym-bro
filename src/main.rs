use axum::{Router, routing::get};

mod database;
mod training_session;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut training_builder = training_session::TrainingSessionBuilder::new();
    training_builder.set_notes(String::from("some notes"));

    training_builder.set_date();
    let weekday_in_russian = training_builder.week_day_russian().unwrap();
    println!("Russian weekday is {weekday_in_russian}");

    let training = match training_builder.build() {
        Some(value) => value,
        None => todo!(),
    };

    println!("Training is {training:#?}");

    let pool = database::create_poll().await?;
    println!("✅ Connected to DB");

    Ok(())

    // let app = Router::new().route("/", get(|| async { "Gym Bro!" }));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
