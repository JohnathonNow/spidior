spidior
=======

`spidior` is a command line utility for performing `sed`-like substitutions on source code files that aims to augment regular expressions with semantic information parsed from the code it is operating on.

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
spidior 0.1.1
John Westhoff <johnjwesthoff@gmail.com>

USAGE:
    spidior [FLAGS] [OPTIONS] [query]

ARGS:
    <query>    The query string for find/replace for each file we find in the input, required if `dump` is not set

FLAGS:
    -d, --dump        Whether we should just dump info without replacing
    -h, --help        Prints help information
    -i, --in-place    Whether we should edit files in place or print to stdout
    -I, --interactive Whether we are are interactively replacing things or not
    -n, --nfa         Whether we should print info about the regex nfa
    -r, --recursive   Whether we should search recursively
    -V, --version     Prints version information

OPTIONS:
    -p, --path <path>    The path to the files we are reading [default: .]

```

Note that right now the program isn't complete. Currently, the following operations are supported:
 - Basic regex operations (concatenation, conjunction, and star [and also plus])
 - Grouping, with backreferences for replacements only
 - Sets and negative sets, but only ranges and explicit characters (e.g. [a-z] or [^xyz] but not \\w or \[\[:upper:]])
 - And most importantly, special queries about identifiers within input programs
    - Currently these queries are put between double square brackets, with a comma separate list of criteria
       - The supported criteria are `name=$NAME` where $NAME is the name of the identifier you are grepping for, `type=$TYPE` where $TYPE is the type of the identifier you are grepping for, and `pos=$POS:$LEN` where $POS is the position into the string to match on for length $LEN.

If the `--dump` argument is used, rather than make any replacements, `spidior` will simply
print out the findings of its lightwight parses from running on the files in the specified path.

Example
-------

Consider this input file, `identifiers.java`:

```java
public class LightningOvercharge extends Lightning {
    int charge = 0;
    public LightningOvercharge() {
        charge = 0;
    }

    double number;
    @Override
    public void onSpawn(Session me) {
        number = 1;
        me.x = 0;
    }
}
```

In the `onSpawn` method, the `Session` input parameter should not be named `me`, so let's fix that and change it to `sess`.
We can run the following command: `spidior -p identifiers.java '%s/[[type=Session]]/sess/g`

The result of this command will be:

```java
public class LightningOvercharge extends Lightning {
    int charge = 0;
    public LightningOvercharge() {
        charge = 0;
    }

    double number;
    @Override
    public void onSpawn(Session sess) {
        number = 1;
        sess.x = 0;
    }
}
```

Similarly we can run: `spidior -p identifiers.java '%s/[[type=double,name=number]]/spawnFlag/g`
to change the previous result into:

```java
public class LightningOvercharge extends Lightning {
    int charge = 0;
    public LightningOvercharge() {
        charge = 0;
    }

    double spawnFlag;
    @Override
    public void onSpawn(Session sess) {
        spawnFlag = 1;
        sess.x = 0;
    }
}
```

Note that this changed both the declaration and the usage of the variable `number`.

Lightweight Parsers
-------------------

Powering `spidior` is a set of language-specific lightweight parsers. Currently, `spidior` requires the ability to parse function declarations, and identifier declaration _and_ usage in order to support operating a language. Right now only a "C-like" parser is written, and it
is very overly-enthusiastic - it identifies many things as identifiers that are, in fact, not identifiers. In practice this ends up being OK, because its mistakes end up including keywords as either the type of the name of the identifier, so no real-world replace operation would be foiled by this overzealousness.

As an example, here is the result of running `spidior --dump -p identifiers.java`:

```rust
Parsing file identifiers.java
        Functions: [Function { name: "LightningOvercharge" }, Function { name: "onSpawn" }]
        Identifiers: [Identifier { name: "com", typ: "static", start: 67, end: 70 }, Identifier { name: "com", typ: "static", start: 232, end: 235 }, Identifier { name: "com", typ: "static", start: 273, end: 276 }, Identifier { name: "com", typ: "static", start: 316, end: 319 }, Identifier { name: "com", typ: "static", start: 361, end: 364 }, Identifier { name: "LightningOvercharge", typ: "class", start: 414, end: 433 }, Identifier { name: "charge", typ: "int", start: 462, end: 468 }, Identifier { name: "charge", typ: "int", start: 517, end: 523 }, Identifier { name: "number", typ: "double", start: 547, end: 553 }, Identifier { name: "me", typ: "Session", start: 601, end: 603 }, Identifier { name: "number", typ: "double", start: 615, end: 621 }, Identifier { name: "me", typ: "Session", start: 635, end: 637 }, Identifier { name: "me", typ: "Session", start: 635, end: 637 }]
```

It correctly identifies the two functions in the source file, but it finds far many variables than actually are real - it found quite a few uses of the "variable" `com` of the "type" `static`. Again in reality you would never try to replace on identifiers of type `static` since that isn't a type, so this isn't an immediate issue. 
