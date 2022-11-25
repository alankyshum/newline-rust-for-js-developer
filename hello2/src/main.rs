#[path = "./modules/try_syntax.rs"] // NOTE Tell Rust to look at this path
mod try_syntax; // NOTE Tell Rust to import the module
use try_syntax::try_syntax; // NOTE Tell Rust to make try_syntax available as namespace

#[path = "./modules/list-files.rs"]
mod list_files;
use list_files::list_files;

use std::env;
use std::fs;

fn main() {
    let arg_value = match env::args().nth(1) {
        Some(input) => input, // NOTE Matching any value
        None => panic!("Try some existing options"),
    };

    match arg_value.as_str() { // NOTE String is different from &str, a resolved pointer to a string value
        /* RUN: `cargo run -- try-syntax` */
        "try-syntax" => {
            println!("Executing try_syntax()");
            try_syntax()
        },
        /* RUN: `cargo run -- list-files ./` */
        "list-files" => {
            println!("Executing list_files()");
            list_files()
        },
        /* RUN: `cargo run -- ./Cargo.toml` */
        &_ => {
            println!("Executing words counter of given file name");

            let file_contents = match fs::read_to_string(arg_value) {
                Ok(contents) => contents,
                Err(err) => panic!("{}", err), // NOTE catch
            };

            let count_of_words = file_contents.split_whitespace().count();
            println!("Number of words: {}", count_of_words)
        }
    }
}
