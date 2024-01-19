fn main() {
    println!("My first word");
}

// Anatomy of the Rust Program.

// fn main() {}
// These lines define a function named main.Here, the first line declares a function named main that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses ().
// The function body is wrapped in {}. Rust requires curly brackets around all function bodies.
// It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

// println!("my first word");
//println! calls a rust macro. the ! indicates that you are calling a rust macro rather than a normal function.
//  we pass a string "My first word" as an argument to println! and the string is printed to the screen.
//  we end the line with a semi colon which is an indication that another line is about to begin.
