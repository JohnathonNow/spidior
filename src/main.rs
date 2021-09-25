use clap::Clap;
use languages::clike::Clike;
use languages::parsing::*;
use std::{error::Error, fs};
use walkdir::WalkDir;
#[macro_use]
extern crate lalrpop_util;

mod editing;
mod languages;
mod nfa;
mod regex2nfa;
mod regexparser;

use crate::regex2nfa::build_nfa;

#[derive(Clap)]
#[clap(version = "0.1.1", author = "John Westhoff <johnjwesthoff@gmail.com>")]
struct Opts {
    /// The path to the files we are reading
    #[clap(short, long, default_value = ".")]
    path: String,
    /// The query string for find/replace for each file we find in the input, required if `dump` is not set
    #[clap(required_unless_present("dump"))]
    query: Option<String>,
    /// Whether we should edit files in place or print to stdout
    #[clap(short, long)]
    in_place: bool,
    /// Whether we should just dump info without replacing
    #[clap(short, long)]
    dump: bool,
    /// Whether we should print info about the regex nfa
    #[clap(short, long)]
    nfa: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    if opts.dump {
        dump(opts)
    } else {
        replace(opts)
    }
}

fn dump(opts: Opts) -> Result<(), Box<dyn Error>> {
    let c = Clike {};
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
                println!("\tFunctions: {:?}", c.read_functions(&contents));
                println!("\tIdentifiers: {:?}", c.read_identifiers(&contents));
            }
        }
    }
    Ok(())
}

fn replace(opts: Opts) -> Result<(), Box<dyn Error>> {
    let replace = regexparser::parse(&opts.query.unwrap())?;
    if opts.nfa {
        let (nfa, _start, _end) = build_nfa(replace.clone().find);
        println!("NFA is `{:?}`", nfa);
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
