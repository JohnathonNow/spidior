//! For parsing out statements of the form
//! LOCATIONs/REGEX/REPLACEMENT/G
//!     where [LOCATION] can be:
//!         % or PATH/TO/FILE:
//!     REGEX is a regex parsed by [reg.lalrpop]
//!     REPLACEMENT is a regex parsed by [replace.lalrpop]
//!     G is either g or the empty string
use super::ast;

pub fn parse(text: &str) -> Result<ast::ReplaceUnparsed, ()> {
    Err(())
}