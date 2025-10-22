mod training;

fn main() {
    let training = training::TrainingSession::new();
    println!("training {training:?}");
}
