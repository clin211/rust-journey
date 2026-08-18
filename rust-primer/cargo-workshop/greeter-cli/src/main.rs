use greeter_core::build_greeter;
use rand::{self, RngExt};

fn main() {
    println!("Hello, world!");
    let names = ["Rustacean", "Ferric", "clin"];
    let mut rng = rand::rng();
    let index = rng.random_range(0..names.len());

    let picked = names[index];
    let message = build_greeter(picked);
    println!("{}", message)
}
