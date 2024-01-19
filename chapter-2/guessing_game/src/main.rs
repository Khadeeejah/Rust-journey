use std::io;
// input/output library scope from the standar library
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // This means we have created a mutable variable,"guess" that is currently bound to a new empty instance of a string.

    io::stdin()
        .read_line(&mut guess)
        //    this is to take whatever the user types into standard input and append that into a string
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
