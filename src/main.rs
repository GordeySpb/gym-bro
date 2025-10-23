mod training_session;

fn main() {
    let training = training_session::TrainingSessionBuilder::new();
    println!("training {training:#?}");
    
    // TODO:handle it correct
    let weekday_in_russian = training.week_day_russian().unwrap();
    println!("Russian weekday is {weekday_in_russian}")
}
