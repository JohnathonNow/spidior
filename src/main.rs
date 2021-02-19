use std::fs;
use walkdir::WalkDir;
use clap::Clap;
#[macro_use] extern crate lalrpop_util;

mod regexparser;
mod languages;
mod editing;

use languages::clike::Clike;
use languages::parsing::{Functions, Identifiers};



#[derive(Clap)]
#[clap(version = "0.1.0", author = "John Westhoff <johnjwesthoff@gmail.com>")]
struct Opts {
    /// The path to the files we are reading
    #[clap(short, long, default_value = ".")]
    path: String,
    /// The query string for find/replace for each file we find in the input
    query: String,
}


fn main() -> Result<(), ()> {
    let opts: Opts = Opts::parse();
    match regexparser::parse(&opts.query) {
        Ok(replace) => println!("Query parsed successfully {:?}", replace),
        Err(e) => println!("Could not parse query {:?}", e),
    }
    
    for entry in WalkDir::new(opts.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {

                let f_name = entry.file_name().to_string_lossy();
                println!("Parsing file {}", f_name);
                let clike = Clike { };
                println!("{:?}\n", clike.read_functions(&contents));
                println!("{:?}\n", clike.read_identifiers(&contents));
            }
        }
    }

    Ok(())
}
