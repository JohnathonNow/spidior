use walkdir::WalkDir;
use std::fs;

mod parsing;
mod java;

use parsing::{Functions, Identifiers};
use java::Java;

fn main() -> Result<(), ()> {
    for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

        let path = entry.path();
        if path.is_file() {
            let contents = fs::read_to_string(path).expect("");
             
            let f_name = entry.file_name().to_string_lossy();
            println!("{}\n{}", f_name, contents);
            println!("{:?}\n", Java::read_functions(&contents));
            println!("{:?}\n", Java::read_identifiers(&contents));
        }
    }

    Ok(())
}
