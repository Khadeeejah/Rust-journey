fn main() {
    let x = 3;
    let x = x + 2;

    {
        let x = x * 5;
        println!("The fist value of x is : {x}");
    }

    println!("The second value of x is : {x}");
}

// This program first binds x to a value of 5.
// Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6.
// Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12.
//  When that scope is over, the inner shadowing ends and x returns to being 6. When we run this program, it will output the following:
