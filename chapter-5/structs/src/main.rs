//   a struct is a custom data types that let you package  and name together multiple related value that makes up a meanignful group.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("anothername@gmail");
}

// A build_user function that uses field init shorthand because the username and email parameters have the same name as struct fields
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// creating instances from other instances

fn main() {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotherexample.com"),
        sign_in_count: user1.sign_in_count,
    };
}

// we can also re-write  the above with less code Note: the .. syntaxspecifies that the remaining not explicitly set should have the same value as the fields in the given instance.

fn main() {
    let user2 = User {
        email: String::from("anotheremail.com"),
        ..user1
    };
}

// Using Tuple Structs Without Named Fields to Create Different Types
// Struct Colour ( i32,i32,i32);
// Struct Point ( i32,i32,i32);

fn main(){
    let black = Colour (0,0,0);
    let origin = Point (0,0,0);
}


// Method syntax
