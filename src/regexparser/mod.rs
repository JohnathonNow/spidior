use self::ast::{Items, Replace};
use std::error::Error;

pub mod ast;
mod parsecommand;
mod parsereplacement;

lalrpop_mod!(reg, "/regexparser/reg.rs");
lalrpop_mod!(set, "/regexparser/set.rs");
lalrpop_mod!(pub query, "/regexparser/query.rs");
lalrpop_mod!(location, "/regexparser/location.rs");

/// For parsing out statements of the form
/// LOCATIONs/REGEX/REPLACEMENT/G
///
/// The difference between this function and `parsecommand::parse`
/// is that the `parsecommand::parse` parses the commands into
/// the portions that make up the command. This function takes
/// things a step further by then parsing out the individual
/// components with their respective parsers.
///
///
/// # Arguments
///
/// * `text` - A string slice that contains the command to be parsed
///
/// # Returns
///
/// A Result<Replace, Box<dyn Error>>, where on success, it returns a
/// Replace containing the LOCATION, REGEX, REPLACEMENT, and
/// whether it is global or not (ends with a g)
pub fn parse(text: &str) -> Result<ast::Replace, Box<dyn Error>> {
    let ru = parsecommand::parse(text)?;
    let location = location::LocationParser::new()
        .parse(&ru.location)
        .map_err(|x| format!("Failed to parse location: {}", x))?;
    let find = if ru.find.is_empty() {
        Box::new(ast::Regex::Simple(Box::new(ast::Simple::Basic(Box::new(ast::Basic::Elementary(Box::new(ast::Elementary::Nothing)))))))

    } else {
        reg::RegexParser::new()
        .parse(&ru.find)
        .map_err(|x| format!("Failed to parse regex: {}", x))?
    };
    let replace = parsereplacement::parse(&ru.replace)?;
    Ok(Replace {
        location,
        find,
        replace: Box::new(replace),
        global: ru.global,
    })
}

pub fn parse_rename(fromname: &str, rename: &str) -> Result<ast::Replace, Box<dyn Error>> {
    let location = Box::new(ast::Location::All);
    let find = reg::RegexParser::new()
        .parse(fromname)
        .map_err(|x| format!("Failed to parse filename regex: {}", x))?;
    let replace = parsereplacement::parse(rename)?;
    Ok(Replace {
        location,
        find,
        replace: Box::new(replace),
        global: true,
    })
}

pub fn parse_set(s: String) -> Box<Items> {
    set::ItemsParser::new().parse(&s).unwrap()
}

#[test]
fn parsing_reg() {
    assert!(reg::RegexParser::new().parse("a|b|c").is_ok());
    assert!(reg::RegexParser::new().parse("abc").is_ok());
    assert!(reg::RegexParser::new().parse("ab-c").is_ok());
    assert!(reg::RegexParser::new().parse("a=bc").is_ok());
    assert!(reg::RegexParser::new().parse("\\-").is_ok());
    assert!(reg::RegexParser::new().parse("[a-c]").is_ok());
    assert!(reg::RegexParser::new().parse("\\n").is_ok());
    assert!(reg::RegexParser::new().parse("[[name=x,type=int]]").is_ok());
    assert!(reg::RegexParser::new().parse("[[functions]]").is_ok());
    assert!(reg::RegexParser::new().parse("a|b|(").is_err());
    assert!(reg::RegexParser::new().parse("[[]]").is_err());
}

#[test]
fn parsing_set() {
    assert!(set::ItemsParser::new().parse("abc").is_ok());
    assert!(set::ItemsParser::new().parse("a-c").is_ok());
    assert!(set::ItemsParser::new().parse("abc-d").is_ok());
    assert!(set::ItemsParser::new().parse("ab-cd").is_ok());
    assert!(set::ItemsParser::new().parse("\\n").is_ok());
    assert!(set::ItemsParser::new().parse("name=x,type=int").is_ok());
    assert!(set::ItemsParser::new().parse("functions").is_ok());
    assert!(set::ItemsParser::new().parse("fun").is_ok());
    assert!(set::ItemsParser::new().parse("abc-").is_err());
}

#[test]
fn parsing_query() {
    assert!(query::QueriesParser::new().parse("name=x").is_ok());
    assert!(query::QueriesParser::new().parse("name=x,type=int").is_ok());
    assert!(query::QueriesParser::new().parse("functions").is_ok());
    assert!(query::QueriesParser::new().parse("fun").is_err());
}
#[test]
fn parsing_location() {
    assert!(location::LocationParser::new().parse("<../../../>").is_ok());
    assert!(location::LocationParser::new().parse("{function}").is_ok());
    assert!(location::LocationParser::new().parse("{function}|l1-5").is_ok());
    assert!(location::LocationParser::new().parse("{function}&l1-5").is_ok());
    assert!(location::LocationParser::new().parse("^{function}|l1-5").is_ok());
    assert!(location::LocationParser::new().parse("(^{function}|l1-5)").is_ok());
    assert!(location::LocationParser::new().parse("l0-2").is_ok());
    assert!(location::LocationParser::new().parse("c0-2").is_ok());
    assert!(location::LocationParser::new().parse("%").is_ok());
    assert!(location::LocationParser::new().parse("%:").is_err());
}

#[test]
fn parsing_entire() {
    assert!(parse("%s/westoff/Westhoff").is_err());
    assert!(parse("%s/westoff/Westhoff/").is_ok());
    assert!(parse("<mod.rs>s/jon/John/g").is_ok());
}
