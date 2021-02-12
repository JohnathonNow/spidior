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

Run `./target/debug/spidior` in the directory of your source files.
In the future, support for specifying directories and regexes will be
added. For now, `spidior` just attempts to locate function declarations
and uses of identifiers.
