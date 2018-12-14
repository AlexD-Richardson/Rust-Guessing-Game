use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 10);

    println!("Guess the number!");

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
