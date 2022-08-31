// Unrecoverable Errors with panic!

fn main() {
    panic!("crash and burn");
} // for debugging run `RUST_BACKTRACE=1 cargo run`

// Recoverable Errors with Result
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    // Traditional
    let greeting_file_1 = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // With pattern matching
    let greeting_file_2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts
    let greeting_file_3 = File::open("hello.txt").unwrap(); //panic

    let greeting_file_4 = File::open("hello.txt").expect("hello.txt should be included in this project"); //expect


}
