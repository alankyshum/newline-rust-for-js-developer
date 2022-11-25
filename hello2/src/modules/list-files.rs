use std::env;
use std::fs;

pub fn list_files() {
    let dirname = parse_dirname();
    let mut entries = Vec::new();

    for result in fs::read_dir(dirname).unwrap() {
        let item = result.unwrap();
        let path_name = item.path().display().to_string();
        let entry = Entry::new(path_name);
        entries.push(entry);
    }

    for entry in entries {
        entry.display();
    }
}

fn parse_dirname() -> String {
    match env::args().nth(2) {
        Some(input) => input,
        None => String::from("./"),
    }
}

// NOTE: struct = interface
struct Entry {
    path: String,
}

// NOTE: impl = class
impl Entry {
    fn display(&self) {
        println!("{}", self.path);
    }

    fn new(path: String) -> Entry {
        // NOTE last expresion === return value
        Entry { path } // NOTE ~JS, key===value name, then we can shorthand it
    }
}
