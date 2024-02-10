use std::io;

fn fib(n: u128) -> u128 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Type \"end\" to end the program");

    loop {
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "end" {
            break;
        }

        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib(n));
    }
}
