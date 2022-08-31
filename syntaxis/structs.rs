// First example

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_with_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main() {
    // Basice instanciation
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");


    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;

    /* It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes.
    The following will not work:
        let user3 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    */
}
