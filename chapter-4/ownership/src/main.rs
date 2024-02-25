// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}

// Build a new "Car" using the values of three input arguments
// - Color of the car (String)
// - Transmission type (enum)
// - Convertible (boolean, true if the car is a convertible)
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Create a new "Car" instance with requested characteristics
    // - Corrected code: return a "Car" struct
    // - Bind first three fields to value of corresponding input argument
    // - Set mileage to 0
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    }
}

fn main() {
    // Order three cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );
}

fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn main() {
    let s1 =String::from("hello")
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2,len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}