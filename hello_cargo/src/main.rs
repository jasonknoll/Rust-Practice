use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // making sure it's an int yo
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed some bullshit: {guess}");

    // compare
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small bitch"),
        Ordering::Greater => println!("too big bitch"),
        Ordering::Equal => println!("we're equal mistuh white"),
    }

    println!("{secret_number}")
}
