use std::fs;
use walkdir::WalkDir;

mod languages;

use languages::clike::Clike;
use languages::parsing::{Functions, Identifiers};

fn main() -> Result<(), ()> {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            let contents = fs::read_to_string(path).expect("");

            let f_name = entry.file_name().to_string_lossy();
            let clike = Clike { };
            println!("{}\n{}", f_name, contents);
            println!("{:?}\n", clike.read_functions(&contents));
            println!("{:?}\n", clike.read_identifiers(&contents));
        }
    }

    Ok(())
}

