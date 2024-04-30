// Let’s make a new binary project with Cargo called rectangles that will take the width and
// height of a rectangle specified in pixels and calculate the area of the rectangle
// fn main() {
//     let width1 = 50;
//     let height1 = 20;

//     println!(
//         "The area of rectangle is {} square pixel",
//         area(width1, height1)
//     );
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//  another version using tuples

// fn main() {
//     let rec1 = (50, 20);

//     print!(" The area of rectangle is {} square pixel", area(rec1));
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

// Refactoring with Structs: Adding More Meaning

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print!(" the area of rectangle is {} square pixels", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width + rectangle.height
}

// Method syntax
