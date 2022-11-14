use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    loop {
        print!("Guess the number (1 ~ 100): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid input");

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
