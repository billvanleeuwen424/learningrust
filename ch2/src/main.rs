use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your gues:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line"); // crash if error

    println!("You guessed: {guess}");
}
