#[path = "./util.rs"]
mod util;

use self::util::{print_stupid_word_1, print_stupid_word_2};

pub fn try_syntax() {
    /* ---------------------------------- Print --------------------------------- */
    println!("---------\n(1) Print new lines");
    println!("Hello, world!");

    /* -------------------------------- Variables ------------------------------- */
    println!("---------\n(2) Variables");
    let mut value = 10;
    let greeting = "Hello!";
    let mut counter = 0;

    counter = counter + 1;
    value += 1;

    println!(
        "Greeting: {}; counter + value = {}",
        greeting,
        counter + value
    );

    /* --------------------------------- Switch --------------------------------- */
    println!("---------\n(3) Switch case");

    let stone = "Thunder stone";

    match stone {
        "Thunder stone" => println!("Jolteon!"),
        "Thunder stone 2" => println!("2 Jolteon!"),
        "Thunder stone 3" => println!("3 Jolteon!"),
        _ => println!("Default"),
    }

    /* -------------------- Call functions from other modules ------------------- */
    println!("---------\n(4) Call external functions");
    print_stupid_word_1();
    print_stupid_word_2();

    /* ---------------------------------- Array --------------------------------- */
    println!("---------\n(5) Arrays and vectors");
    let mut languages = vec!["javascript", "rust"];
    languages.push("rust");

    for language in languages {
        println!("Programing language: {}", language);
    }
}
