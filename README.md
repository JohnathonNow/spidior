spidior
=======

Status
------
![Build Status](https://github.com/JohnathonNow/spidior/workflows/Rust/badge.svg)

Building
--------

Install a recent stable [rust](https://rustup.rs/), clone this repo,
and run `cargo build`.

Running
-------

The following is the --help output for `spidior`, which shows you how to run it.

```
USAGE:
    spidior [OPTIONS] <query>

ARGS:
    <query>    The query string for find/replace for each file we find in the input

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    The path to the files we are reading [default: .]
```

Note that right now the program isn't complete. Currently, the query only supports
basic regex searches and plaintext replacement strings.
