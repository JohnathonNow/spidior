//! For parsing out statements of the form
//! LOCATIONs/REGEX/REPLACEMENT/G
//!     where [LOCATION] can be:
//!         % or PATH/TO/FILE:
//!         parsed by location.lalrpop
//!     REGEX is a regex parsed by [reg.lalrpop]
//!     REPLACEMENT is a regex parsed by [replace.lalrpop]
//!     G is either g or the empty string
use super::ast;

pub fn parse(text: &str) -> Result<ast::ReplaceUnparsed, ()> {
    let (location, start) = parse_portion(text, 0)?;
    if location.chars().last().ok_or(())? != 's' {
        return Err(());
    }
    let (regex, start) = parse_portion(text, start)?;
    let (replacement, start) = parse_portion(text, start)?;
    let rest = &text[start..];
    let global = if rest.len() == 0 {
        Ok(false)
    } else {
        if rest.len() != 1 {
            Err(())
        } else {
            if rest.chars().next().ok_or(())? == 'g' {
                Ok(true)
            } else {
                Err(())
            }
        }
    }?;
    Ok(ast::ReplaceUnparsed {
        find: regex,
        replace: replacement,
        location: location,
        global: global,

    })
}

/// Parses text until it finds an unescaped / chracter
/// # Arguments
///
/// * `text` - A string slice that contains the command to be parsed
///
/// # Returns
///
/// A Result<(String, usize), ()>, where on success, it returns a
/// tuple containing the parsed string and the index of where to start
/// for future parsing.
fn parse_portion(text: &str, start: usize) -> Result<(String, usize), ()> {
    let mut escape = false;
    for (i, c) in text.chars().enumerate().skip(start) {
        match c {
            '\\' => escape = !escape,
            '/' => {
                if !escape {
                    return Ok((text[0..i].to_string(), i + 1));
                }
            }
            _ => escape = false,
        }
    }
    Err(())
}
