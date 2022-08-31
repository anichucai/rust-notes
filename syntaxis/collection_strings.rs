fn main() {
    // Defining

    let mut s2 = String::new();

    let data = "initial contents";

    let s1 = data.to_string();

    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    // Updating

    let mut s_update = String::from("foo");

    s_update.push_str("bar");
    s_update.push_str(s2);
    s_update.push('l');

    let s5 = s1 + &s2; // note s1 has been moved here and can no longer be used


    // Formatting

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toc = String::from("toe");


    let s6 = format!("{}-{}-{}", tic, tac, toe);

    // Indexing

    let hello = "Здравствуйте";
    let answer = &hello[0]; // hello[0] will not work bc hello is inmutable

    // Slicing

    let answer = &hello[0..4];

    // Iterating

    for c in "123455".chars() {
        println!("{}", c);
    }

    for b in "123455".bytes() {
        println!("{}", b);
    }

}
