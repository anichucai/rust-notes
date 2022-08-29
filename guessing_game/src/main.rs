/*
To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
The io library comes from the standard library, known as std:
By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.
The prelude is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
*/
use std::io;

/*
"crate" is a collection of Rust source code files.
The project we’ve been building is a "binary crate", which is an executable.
The rand crate is a "library crate", which contains code intended to be used in other programs and can't be executed on its own.
Should be added in Cargo.toml file as a dependency with its version (semantic versioning).
When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io.
Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.
*/
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // We use the let statement to create the variable.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // We add mut before the variable name and after the let statement to create a mutable variable.
        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        let mut guess = String::new();

        /*read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
        Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
        Result's variants are Ok and Err. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value.
        The Err variant means the operation failed, and Err contains information about how or why the operation failed.
        An instance of Result has an expect method that you can call.
        If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect.
        If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it.
        If you don’t call expect, the program will compile, but you’ll get a warning.*/
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
