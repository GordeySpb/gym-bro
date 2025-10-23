mod training_session;

fn main() {
    let training = training_session::TrainingSessionBuilder::new();
    println!("training {training:#?}");
}
