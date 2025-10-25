mod training_session;

fn main() {
    let mut training = training_session::TrainingSessionBuilder::new();
    training.set_notes(String::from("some notes"));
    println!("training {training:#?}");

    training.set_date();
    let weekday_in_russian = training.week_day_russian().unwrap();
    println!("Russian weekday is {weekday_in_russian}");

    println!("Training is {training:#?}")
}
