use clap::Clap;
use languages::clike::Clike;
use languages::parsing::*;
use std::{error::Error, fs};
use walkdir::WalkDir;
use std::io::{self, BufRead};

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
    #[clap(short = 'q', long, required_unless_present("dump"))]
    query: Option<String>,
    /// Whether we are are interactively replacing things or not
    #[clap(short = 'I', long)]
    interactive: bool,
    /// Whether we should edit files in place or print to stdout
    #[clap(short, long)]
    in_place: bool,
    /// Whether we should just dump info without replacing
    #[clap(short, long)]
    dump: bool,
    /// Whether we should print info about the regex nfa
    #[clap(short, long)]
    nfa: bool,
    /// Whether we should search recursively
    #[clap(short, long)]
    recursive: bool,
}

fn ask(replace: &str, with: &str) -> bool {
    println!("Replace:\n{}\nWith:\n{}\n?", replace, with);
    let mut answer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut answer).unwrap();
    return answer.to_lowercase().starts_with("y");
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
    for entry in get_dir_iter(opts.recursive, &opts.path)
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

    for entry in get_dir_iter(opts.recursive, &opts.path)
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                let f_name = entry.file_name().to_string_lossy();
                let res = nfa::replacer::replace(&contents, replace.clone(), if opts.interactive { ask } else { |x, y| true} )?;
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

fn get_dir_iter(recursive: bool, path: &str) -> impl Iterator<Item=walkdir::DirEntry> {
    let mut iter = WalkDir::new(path);
    if !recursive {
        iter = iter.max_depth(1);
    }
    iter.follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
}