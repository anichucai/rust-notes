use std::collections::HashMap;

fn main() {
    // Defining

    let mut scores = HashMap::new();

    // Inserting

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    scores.insert(field_name, field_value); // field_name and field_value are invalid at this point, try using them and see what compiler error you get!

    // Overwriting a Value

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Adding a Key and Value Only If a Key Isn’t Present

    scores.entry(String::from("Blue")).or_insert(50);

    // Updating a Value Based on the Old Value
    let count = scores.entry(String::from("Blue")).or_insert(0);
    *count += 1;
}
