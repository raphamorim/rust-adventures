use std::io;
extern crate docker;

fn main() {
    println!("Read line:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse()
        .expect("Failed to parse");

    scoring(guess);
    docker::public_function();
}

fn scoring(n: u32) -> u32 {
    n * 2
}
