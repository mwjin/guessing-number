use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    println!("The secret number is: {secret_number}");
}
