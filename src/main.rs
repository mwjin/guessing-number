use rand::Rng;
use std::io::{self, Write};
fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    print!("Enter your number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    let number: u32 = input.trim().parse().expect("Enter a valid number!");
    println!("Your input number: {number}");
}
