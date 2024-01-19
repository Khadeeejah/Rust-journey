// use std::io;
// // input/output library scope from the standar library
// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();
//     // This means we have created a mutable variable,"guess" that is currently bound to a new empty instance of a string.

//     io::stdin()
//         .read_line(&mut guess)
//         //    this is to take whatever the user types into standard input and append that into a string
//         .expect("Failed to read line");
//     println!("You guessed: {guess}");
// }

// generating a random number
use rand::Rng;
// this defnes the method the random number generator implements
use std::cmp::Ordering;
// cmp ordering from standard library
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // this function gives us the particular random number generator we are going to use.
    // the gen-range method takes a range expression as an argument aand generates a random number in the range

    println!("Your secret number is {secret_number} ");

    loop {
        // loop keyword creates an ifinite loop, which allows you to keep guessing
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // this function allows the user inputs a number to guess, initially it was a string but i altered it to u32, to make it sure its just a number that can be added
            Ok(num) => num,
            Err(_) => continue,
            // initially this used to be an epect call,
            // but
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                return;
                // the break statement lets the game end after guessing the correct number
            }
        }
    }
}
