use std::error::Error;

use super::ast;

/// For parsing out statements of the form
/// LOCATIONs/REGEX/REPLACEMENT/G
/// # Arguments
///
/// * `text` - A string slice that contains the command to be parsed
///
/// # Returns
///
/// A Result<ReplaceUnparsed, Box<dyn Error>>, where on success, it returns a
/// ReplaceUnparsed containing the LOCATION, REGEX, REPLACEMENT, and
/// whether it is global or not (ends with a g)
pub fn parse(text: &str) -> Result<ast::ReplaceUnparsed, Box<dyn Error>> {
    let (location, start) = parse_portion(text, 0)?;
    if location.chars().last().ok_or("Location empty, expected at least an s")? != 's' {
        return Err("s expected in location".into());
    }
    let (find, start) = parse_portion(text, start)?;
    let (replace, start) = parse_portion(text, start)?;
    let rest = &text[start..];
    let global = if rest.len() == 0 {
        Ok(false)
    } else {
        if rest.len() != 1 {
            Err("Expected exactly one character after the replacement")
        } else {
            if rest.chars().next().ok_or("Really expected at least one last char")? == 'g' {
                Ok(true)
            } else {
                Err("Expected a g")
            }
        }
    }?;
    Ok(ast::ReplaceUnparsed {
        location: location[..location.len() - 1].to_string(),
        find,
        replace,
        global,
    })
}

/// Parses text until it finds an unescaped / chracter
/// # Arguments
///
/// * `text` - A string slice that contains the command to be parsed
/// * `start` - The index in the string to start from
///
/// # Returns
///
/// A Result<(String, usize), Box<dyn Error>>, where on success, it returns a
/// tuple containing the parsed string and the index of where to start
/// for future parsing.
fn parse_portion(text: &str, start: usize) -> Result<(String, usize), Box<dyn Error>> {
    let mut escape = false;
    for (i, c) in text.chars().enumerate().skip(start) {
        match c {
            '\\' => escape = !escape,
            '/' => {
                if !escape {
                    return Ok((text[start..i].to_string(), i + 1));
                }
            }
            _ => escape = false,
        }
    }
    Err("Did not find an unescaped backslash!".into())
}

#[test]
fn parsing_command() {
    assert!(parse("%s/westoff/Westhoff").is_err());
    assert!(parse("").is_err());
    assert!(parse("%b/westoff/Westhoff/").is_err());
    let parsed = parse("%s/westoff/Westhoff/");
    assert!(parsed.is_ok());
    let x = parsed.unwrap();
    assert_eq!(x.location, "%");
    assert_eq!(x.find, "westoff");
    assert_eq!(x.replace, "Westhoff");
    assert_eq!(x.global, false);
    let parsed = parse("mod.rs:s/jon/John/g");
    assert!(parsed.is_ok());
    let x = parsed.unwrap();
    assert_eq!(x.location, "mod.rs:");
    assert_eq!(x.find, "jon");
    assert_eq!(x.replace, "John");
    assert_eq!(x.global, true);
}
