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
USAGE:
    spidior [OPTIONS] <query>

ARGS:
    <query>    The query string for find/replace for each file we find in the input

FLAGS:
    -h, --help       Prints help information
    -i, --in-place    Whether we should edit files in place or print to stdout
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    The path to the files we are reading [default: .]
```

Note that right now the program isn't complete. Currently, the following operations are supported:
 - Basic regex operations (concatenation, conjunction, and star [and also plus])
 - Grouping, with backreferences for replacements only
 - Sets and negative sets, but only ranges and explicit characters (e.g. [a-z] or [^xyz] but not \\w or \[\[:upper:]])
 - And most importantly, special queries about identifiers within input programs
    - Currently these queries are put between double square brackets, with a comma separate list of criteria
       - The supported criteria are `name=$NAME` where $NAME is the name of the identifier you are grepping for, and `type=$TYPE` where $TYPE is the type of the identifier you are grepping for

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