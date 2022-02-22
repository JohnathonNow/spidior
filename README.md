spidior
=======

`spidior` is a command line utility for performing `sed`-like substitutions on source code files that aims to augment regular expressions with semantic information parsed from the code it is operating on.

Status
------
![Build Status](https://github.com/JohnathonNow/spidior/workflows/Rust/badge.svg)

Note `spidior` is still a very early stage project, so it should ***NOT*** be used without backing up files first.

Building
--------

Install a recent stable [rust](https://rustup.rs/), clone this repo,
and run `cargo build`.

Running
-------

The following is the --help output for `spidior`, which shows you how to run it.

```
spidior 0.2.3
John Westhoff <johnjwesthoff@gmail.com>

USAGE:
    spidior [OPTIONS]

FLAGS:

OPTIONS:
    -p, --path <path>      The path to the files we are reading [default: .]
    -q, --query <query>    The query string for find/replace for each file we find in the input, required if `dump` is not set
    -R, --rename <RENAME>  Optional regex for renaming files with any matches
    -d, --dump             Whether we should just dump info without replacing
    -h, --help             Prints help information
    -i, --in-place         Whether we should edit files in place or print to stdout
    -I, --interactive      Whether we are are interactively replacing things or not
    -n, --nfa              Whether we should print info about the regex nfa
    -r, --recursive        Whether we should search recursively
    -V, --version          Prints version information

```

If the `--dump` argument is used, rather than make any replacements, `spidior` will simply
print out the findings of its lightwight parses from running on the files in the specified path.
Otherwise, a query must be specified with either -q or --query.

### Queries

Queries are very similar to `sed` s commands, and take the form ${LOCATION}${COMMAND}/${FIND}/${REPLACE}/${END}
Where ${LOCATION} is where replacements should be allowed to take place (more on that below), ${COMMAND} is always s,
$(FIND) is a regular expression, ${REPLACE} is a replacement, and ${END} is either nothing or the letter 'g' to allow multiple
replacements on a given line.

#### Locations
A location can be one of several things:  
 - `%` - anywhere in any file the path specifier includes  
 - `<path_suffix>` - anywhere in any file whose path ends in path_suffix  
 - `{function}` - anywhere in any file within a function named function  
 - `cA-B` - anywhere in any file between the Ath (inclusive) and Bth (exclusive) character in the file  
 - `lA-B` - anywhere in any file between the Ath (inclusive) and Bth (exclusive) line in the file  

Locations can also be grouped using parens, unioned with `|`, intersected with `&`, and negated with `^`.
Why ^ instead of !? Well I figured since sets in most regex interpreters use ^ for negation it made sense here.

#### Regex
Regexes follow standard `sed`like syntax, and support the following operations:  
 - Basic regex operations (concatenation, conjunction, and star [and also plus])
 - Grouping with parens
 - Sets and negative sets, but only ranges and explicit characters (e.g. [a-z] or [^xyz] but not \\w or \[\[:upper:]])
 - And most importantly, special queries about identifiers within input programs
    - Currently these queries are put between double square brackets, with a comma separate list of criteria
       - The supported criteria are `name=$NAME` where $NAME is the name of the identifier you are grepping for, `type=$TYPE` where $TYPE is the type of the identifier you are grepping for, and `pos=$POS:$LEN` where $POS is the position into the string to match on for length $LEN.

#### Replacements
A replacement is a string literal that may include backreferences to groups using a backslash followed by a number.

Renaming
--------

In addition to editing files, potentially in place, `spidior` allows for renaming files through the same
substitution commands it can run on text. By passing a `--rename REPLACEMENT` option, any file who
matches the provided query will be renamed to REPLACEMENT. The REPLACEMENT string can use backreferences
to path locations in the Location portion of the query. For instance, to rename any file ending in .java,
one could write:

`spidior -q '<(.*)\.java>s///' --rename '\1_renamed.java'

which will rename every file ending in '.java' to instead end in '_renamed.java'.

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

```json
[{"filename":"identifiers.java","functions":[{"name":"LightningOvercharge","start":507,"end":534},{"name":"onSpawn","start":605,"end":671}],"identifiers":[{"name":"com","type_name":"static","start":67,"end":70},{"name":"com","type_name":"static","start":232,"end":235},{"name":"com","type_name":"static","start":273,"end":276},{"name":"com","type_name":"static","start":316,"end":319},{"name":"com","type_name":"static","start":361,"end":364},{"name":"LightningOvercharge","type_name":"class","start":414,"end":433},{"name":"charge","type_name":"int","start":462,"end":468},{"name":"charge","type_name":"int","start":517,"end":523},{"name":"number","type_name":"double","start":547,"end":553},{"name":"me","type_name":"Session","start":601,"end":603},{"name":"number","type_name":"double","start":615,"end":621},{"name":"me","type_name":"Session","start":635,"end":637},{"name":"me","type_name":"Session","start":635,"end":637}]}]
```

It correctly identifies the two functions in the source file, but it finds far many variables than actually are real - it found quite a few uses of the "variable" `com` of the "type" `static`. Again in reality you would never try to replace on identifiers of type `static` since that isn't a type, so this isn't an immediate issue. 
