fn main() {
    println!("Hello, world!");

    another_function();
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value.

// let x = (let y = 6); fails because you can’t assign a let statement (does not return a value)

fn another_function() {
    println!("Another function.");
}

fn another_function() {
    let y = {
        let x = 3;
        x + 1 // Is a return because it doesn’t have a semicolon at the end
    };

    println!("The value of y is: {y}");
}


fn function_with_return_value() -> i32 {
    5
}

fn function_without_return_value() {
    let x = five();

    println!("The value of x is: {x}");
}
