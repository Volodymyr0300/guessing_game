use std::io;


fn main() {
    let mut guess = String::new();
    // let foo = bar;

    // let foo = 5; // immutable
    // let mut bar = 5; // mutable

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}

