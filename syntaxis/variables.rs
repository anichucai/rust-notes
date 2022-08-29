fn main() {
    /*_______________mutable vs inmutable variables___________________*/
    let y = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    //y = 3 compiler error
    println!("The value of x is: {x}");

    /*_______________constants___________________*/
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    // Constants are valid for the entire time a program runs, within the scope they were declared in.

    /*_______________Shadowing___________________*/
    let x = 5;
    let x = x + 1;
    // Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.
    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
}
