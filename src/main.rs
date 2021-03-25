use std::{error::Error, fs};
use walkdir::WalkDir;
use clap::Clap;
#[macro_use] extern crate lalrpop_util;

mod regexparser;
mod languages;
mod editing;
mod nfa;
mod regex2nfa;




#[derive(Clap)]
#[clap(version = "0.1.0", author = "John Westhoff <johnjwesthoff@gmail.com>")]
struct Opts {
    /// The path to the files we are reading
    #[clap(short, long, default_value = ".")]
    path: String,
    /// The query string for find/replace for each file we find in the input
    query: String,
    /// Whether we should edit files in place or print to stdout
    #[clap(short, long)]
    in_place: bool,
}


fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let replace = regexparser::parse(&opts.query)?;
    
    for entry in WalkDir::new(opts.path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                let f_name = entry.file_name().to_string_lossy();
                let res = nfa::replacer::replace(&contents, replace.clone())?;
                println!("Parsing file {}", f_name);
                if opts.in_place {
                    fs::write(path, &res)?;
                } else {
                    println!("{}", res);
                }
            }
        }
    }

    Ok(())
}
