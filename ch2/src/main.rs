use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let rand_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your gues:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line"); // crash if error

        let guess: u32 = guess.trim().parse().expect("Please enter a number.");

        println!("You guessed: {guess}");

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
