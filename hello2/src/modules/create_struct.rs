use std::env;

pub fn create_new_struct() {
    let dirname = parse_dirname();

    let entry = Entry {
        path: String::from(dirname)
    };

    println!("dirname = entry.path: {:?}", entry.path);
}

fn parse_dirname() -> String {
    match env::args().nth(1) {
        Some(input) => input,
        None => String::from("./"),
    }
}

struct Entry {
    path: String
}