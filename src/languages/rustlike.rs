//! Provides the parser for "rust-like" languagescd 

use super::parsing::{Function, Functions, Identifier, Identifiers};

/// A Functions and Identifiers parser for rustlike languages.
pub struct Rustlike {}

impl Functions for Rustlike {
    /// Parses out function declarations from rust-like code
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that contains the code to be parsed
    ///
    /// # Returns
    ///
    /// A Vec of Function containing information on every function
    /// declared within text
    fn read_functions(&self, _text: &str) -> Vec<Function> {
        todo!("implement this");
    }
}


impl Identifiers for Rustlike {
    /// Parses out identifier uses from rust-like code
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that contains the code to be parsed
    ///
    /// # Returns
    ///
    /// A Vec of Identifier containing information on every use of
    /// an identifier declared within the code
    fn read_identifiers(&self, _text: &str) -> Vec<Identifier> {
        todo!("implement this");

    }
}
