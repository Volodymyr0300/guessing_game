use std::io;


fn main() {
    let mut guess = String::new();
    let foo = bar;

    // The following example shows how to use mut before the variable name to make a variable mutable:
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

