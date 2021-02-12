use std::error::Error;

use super::ast::{ReplaceItem, Replacement};

/// For parsing out the replacement form of a command
/// # Arguments
///
/// * `text` - A string slice that contains the replacement to be parsed
///
/// # Returns
///
/// A Result<Replacement, ()>, where on success, it returns a
/// Replacement containing the set of ReplaceItems
/// that make up the new replacement string
pub fn parse(text: &str) -> Result<Replacement, Box<dyn Error>> {
    let mut v = Vec::new();
    let mut i = 0;
    while i < text.len() {
        let (p, n) = parse_item(text, i)?;
        v.push(p);
        i = n;
    }
    Ok(Replacement {replacements: v})
}

/// Parses text until it finds the end of a ReplaceItem
/// # Arguments
///
/// * `text` - A string slice that contains the replacement to be parsed
/// * `start` - The index in the string to start from
///
/// # Returns
///
/// A Result<(ReplaceItem, usize), ()>, where on success, it returns a
/// tuple containing the parsed ReplaceItem and the index of where to start
/// for future parsing.
fn parse_item(text: &str, start: usize) -> Result<(ReplaceItem, usize), Box<dyn Error>> {
    let mut chars = text.chars().enumerate().skip(start);
    if chars.next().ok_or("Out of characters")?.1 == '\\' {
        // We might be parsing a backreference
        let mut last = 0;
        while let Some((i, c)) = chars.next() {
            if !c.is_digit(10) {
                break;
            } else {
                last = i;
            }
        }
        if last != 0 {
            return Ok((
                ReplaceItem::BackRef(text[start + 1..last+1].parse::<i32>()?),
                last + 1,
            ));
        }
    }
    //If we are here, we are parsing a
    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            return Ok((ReplaceItem::String(text[start..i].to_string()), i));
        }
    }
    Ok((
        ReplaceItem::String(text[start..].to_string()),
        text.len() + 1,
    ))
}

#[test]
fn parsing_replacement() -> Result<(), Box<dyn Error>>{
    let parsed = parse("bob\\\\\\13dole")?;
    if let ReplaceItem::String(s) = parsed.replacements.get(0).ok_or("sad")? {
        assert_eq!(s, "bob");
    } else {
        panic!("Expected a string, but didn't get it");
    }
    if let ReplaceItem::String(s) = parsed.replacements.get(1).ok_or("sad")? {
        assert_eq!(s, "\\\\");
    } else {
        panic!("Expected a string, but didn't get it");
    }
    if let ReplaceItem::BackRef(x) = parsed.replacements.get(2).ok_or("sad")? {
        assert_eq!(*x, 13);
    } else {
        panic!("Expected a string, but didn't get it");
    }
    if let ReplaceItem::String(s) = parsed.replacements.get(3).ok_or("sad")? {
        assert_eq!(s, "dole");
    } else {
        panic!("Expected a string, but didn't get it");
    }
    assert_eq!(parsed.replacements.len(), 4);
    Ok(())
}
