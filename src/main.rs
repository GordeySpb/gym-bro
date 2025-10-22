mod training_session;

fn main() {
    let training = training_session::Training::new();
    println!("training {training:?}");
}
