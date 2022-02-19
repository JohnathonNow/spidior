//! This module is for language specific implementations of the traits in the
//! `parsing` module. These traits are separated out as not all languages
//! have the same features to be extracted. For example,
//! Python has Function names to be extracted, but as its
//! identifiers do not have statically knowable types, it
//! is not Identifiers.
//! C and Java on the other hand have both extractable Functions
//! and Identifiers.

/// Provides traits for parsing different features of source code
pub mod parsing {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Info {
        pub filename: String,
        pub functions: Vec<Function>,
        pub identifiers: Vec<Identifier>,
    }

    impl Info {
        pub fn new(
            filename: String,
            functions: Vec<Function>,
            identifiers: Vec<Identifier>,
        ) -> Self {
            Self {
                filename,
                functions,
                identifiers,
            }
        }
    }

    /// Represents a function in a source code file
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Function {
        /// We only care about named functions - thus, all functions have names
        pub name: String,
    }

    /// Represents an identifier in a piece of code, which has an associated type
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Identifier {
        /// The name of the identifier, which is the set of characters used to refer to it
        pub name: String,
        /// The type of value the identifier represents
        pub type_name: String,
        /// The starting index within the source file this identifier is located at
        pub start: usize,
        /// The index one past the end of the identifier's location
        pub end: usize,
    }

    /// A trait for language processors that support named functions
    pub trait Functions {
        /// Retrieves a vector of all the named functions given a piece of source code
        /// # Arguments
        ///
        /// * `text` - A string slice that contains the source code to be analyzed
        ///
        /// # Returns
        ///
        /// A `Vec<Function>` containing every named function within `text`
        fn read_functions(&self, text: &str) -> Vec<Function>;
    }

    /// A trait for language processors that support tracking identifiers
    pub trait Identifiers {
        /// Retrieves a vector of all the typed identifiers within a piece of source code
        /// # Arguments
        ///
        /// * `text` - A string slice that contains the source code to be analyzed
        ///
        /// A `Vec<Identifier>` containing every named identifier within `text`
        fn read_identifiers(&self, text: &str) -> Vec<Identifier>;
    }

    impl Identifier {
        /// Creates a new Identifier given a set of parameters
        /// # Arguments
        ///
        /// * `text` - A String that contains the name of the identifier
        /// * `typ` - A String that contains the name of the type of the identifier
        /// * `start` - a number representing the where the identifier starts in the code
        /// * `end` - a number representing the where the identifier ends in the code
        ///
        pub fn new(name: String, typ: String, start: usize, end: usize) -> Self {
            Self {
                name,
                type_name: typ,
                start,
                end,
            }
        }
    }

    impl Function {
        /// Creates a new Function given a set of parameters
        /// # Arguments
        ///
        /// * `text` - A String that contains the name of the function
        ///
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }
}
pub mod clike;
pub mod rustlike;