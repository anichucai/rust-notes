/*Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.  */

fn main() {
    // Defining

    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5, 6];

    let mut v3 = Vec::new();

    // Writing

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.pop();

    // Reading

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    let second: Option<&i32> = v2.get(1);
    match second {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element."),
    }

    // Iterating

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v3 {
        *i += 50;
    }
} // <- v* goes out of scope and is freed here
