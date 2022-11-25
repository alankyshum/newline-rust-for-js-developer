#[path = "./modules/try_syntax.rs"]
mod try_syntax;
use try_syntax::try_syntax;

use std::env;
use std::fs;

fn main() {
    let arg_value = match env::args().nth(1) {
        Some(input) => input,
        None => panic!("Try some existing options"),
    };

    match arg_value.as_str() {
        "try-syntax" => {
            println!("Executing try_syntax()");
            try_syntax()
        },
        &_ => {
            println!("Executing words counter of given file name");

            let file_contents = match fs::read_to_string(arg_value) {
                Ok(contents) => contents,
                Err(err) => panic!("{}", err),
            };

            let count_of_words = file_contents.split_whitespace().count();
            println!("Number of words: {}", count_of_words)
        }
    }
}
