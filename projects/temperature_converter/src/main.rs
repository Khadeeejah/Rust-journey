use std::io::stdin;

fn main() {
    println!("What temperature will you like to convert");

    println!("[F] for converting from Celcius to Fareheit");
    println!("[C] for converting from Fareheit to Celcius");

    let mut choice: String = String::new();

    stdin()
        .read_line(&mut choice)
        .expect("Please provide your choice");

    let choice = choice.trim().to_uppercase();

    if choice != "F" && choice != "C" {
        println!("This is not a correct temperature");
        return;
    }

    println!("Provide a temperature");
    let mut temperature: String = String::new();
    stdin().read_line(&mut temperature).unwrap_or_default();

    let temperature: f64 = temperature.trim().parse().unwrap_or_default();

    // Converting from F to C = (45F-32) * 5/9
    //  converting from C to F = (45C x 9/5) + 32

    let new_temperature: f64 = match choice.as_str() {
        "F" => (temperature * (9f64 / 5f64)) * 32 as f64,
        "C" => (temperature - 32f64) * (5f64 / 9f64),
        _ => 0.00,
    };
    println!(
        "The converted temperature is {} {}",
        new_temperature, choice
    )
}
