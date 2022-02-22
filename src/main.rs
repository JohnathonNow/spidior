use clap::Parser;
use languages::clike::Clike;
use languages::parsing::*;
use regexparser::ast;
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

#[derive(Parser)]
#[clap(version = "0.2.3", author = "John Westhoff <johnjwesthoff@gmail.com>")]
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
    /// Optional regex for renaming files with any matches
    #[clap(short = 'R', long)]
    rename: Option<String>,
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

fn ask_rename(replace: &str, with: &str) -> bool {
    eprintln!("Rename: '{}' as '{}'\n?", replace, with);
    let mut answer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut answer).unwrap();
    return answer.to_lowercase().starts_with("y");
}

fn ask(replace: &str, with: &str) -> bool {
    eprintln!("Replace:\n{}\nWith:\n{}\n?", replace, with);
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
    let mut dumps = Vec::new();
    for entry in get_dir_iter(opts.recursive, &opts.path)
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                dumps.push(Info::new(path.to_string_lossy().to_string(), c.read_functions(&contents), c.read_identifiers(&contents)));
            }
        }
    }
    println!("{}", serde_json::to_string(&dumps).unwrap());
    Ok(())
}

fn replace(opts: Opts) -> Result<(), Box<dyn Error>> {
    let replace = regexparser::parse(&opts.query.unwrap())?;
    let mut rename: Option<Vec<ast::Replace>> = None;
    if let Some(rename_str) = opts.rename {
        let mut filenames = vec![];
        replace.location.get_filenames(&mut filenames);
        rename = Some(
            filenames.iter().map(|f| regexparser::parse_rename(f, &rename_str)).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect()
        );
    }
    if opts.nfa {
        let (nfa, _start, _end) = build_nfa(replace.clone().find);
        eprintln!("NFA is `{}`", serde_json::to_string(&nfa).unwrap());
    }

    for entry in get_dir_iter(opts.recursive, &opts.path)
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(contents) = fs::read_to_string(path) {
                let f_name = entry.file_name().to_string_lossy();
                let path_name = path.to_string_lossy().to_string();
                let (res, did_change) = nfa::replacer::replace(&path_name, &contents, replace.clone(), if opts.interactive { ask } else { |_, _| true} )?;
                eprintln!("Parsing file {}", f_name);
                if opts.in_place {
                    fs::write(path, &res)?;
                } else {
                    println!("{}", res);
                }
                if let Some(renames) = &rename {
                    if did_change {
                        for rename in renames {
                            let x = nfa::replacer::replace(&path_name, &path_name, rename.clone(), if opts.interactive { ask_rename } else { |_, _| true} )?;
                            if x.1 {
                                eprintln!("Renaming file '{}' to '{}'", &path_name, x.0);
                                fs::rename(&path_name, x.0)?;
                                break;
                            }
                        }
                    }
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
