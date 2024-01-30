// fn main() {
//     println!("Hello, world!");
//     second_function();
// }

// fn second_function() {
//     println!("Second function!");
// }

// parameters
// fn main() {
//     another_function(4);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// function signature
fn main() {
    print_labelled_measurement(5, 'h');
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
