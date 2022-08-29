fn main() {

    // Keep in mind that Rust is a statically typed language

    /*--------------------------------------------*/
    /*----------------Scalar Types----------------*/
    /*--------------------------------------------*/

    let x1 = 1; // f64

    let x2: i8 = 3; // Signed 8-bit

    let x3: i128 = 3; // Unsigned 128-bit

    let x4: isize = 3; // Signed arch

    /*_______________Floating-Point Types___________________*/
    let y1 = 2.0; // f64

    let y2: f32 = 3.0; // f32

    /*_______________Numeric Operations___________________*/
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    /*_______________The Boolean Type___________________*/
    let t = true;

    let f: bool = false; // with explicit type annotation

   /*_______________The Character Type___________________*/
   let c = 'z';
   let z: char = 'ℤ'; // with explicit type annotation
   let heart_eyed_cat = '😻';


    /*----------------------------------------------*/
    /*----------------Compound Types----------------*/
    /*----------------------------------------------*/

    /*_______________The Tuple Type___________________*/

    //Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;

    println!("The value of x is: {five_hundred}");
    println!("The value of y is: {y}");

    /*_______________The Tuple Type___________________*/
    // Arrays in Rust have a fixed length. Every element of an array must have the same type
    // Allocated in the stack
    let a1 = [1, 2, 3, 4, 5];

    let a2: [i32; 5] = [1, 2, 3, 4, 5];

    let a3 = [3, 3, 3, 3, 3];

    let a4 = [3; 5]; // a3 es ufual a a4

    let first = a1[0];
    let second = a1[1];

}
