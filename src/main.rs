mod training_session;

fn main() {
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
}
