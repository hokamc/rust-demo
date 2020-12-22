use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Input guess:");

    let mut input_text = String::new();

    let random_value = rand::thread_rng().gen_range(1, 101);

    println!("secret number: {}", random_value);

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let guess: u32 = input_text.trim().parse().expect("Not a number");

    println!("You guessed: {}", input_text);

    match guess.cmp(&random_value) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("won"),
    }
}
