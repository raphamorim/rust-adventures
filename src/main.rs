use std::io;

fn main() {
    println!("Read line:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess);

     println!("You guessed: {}", guess);
}

