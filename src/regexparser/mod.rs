mod ast;
mod parsecommand;

lalrpop_mod!(reg, "/regexparser/reg.rs");
lalrpop_mod!(set, "/regexparser/set.rs");
lalrpop_mod!(query, "/regexparser/query.rs");
lalrpop_mod!(location, "/regexparser/location.rs");

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
fn parsing_command() {
    assert!(parsecommand::parse("%s/westoff/Westhoff").is_err());
    let parsed = parsecommand::parse("%s/westoff/Westhoff/");
    assert!(parsed.is_ok());
    let x = parsed.unwrap();
    assert_eq!(x.location, "%");
    assert_eq!(x.find, "westoff");
    assert_eq!(x.replace, "Westhoff");
    assert_eq!(x.global, false);
    let parsed = parsecommand::parse("mod.rs:s/jon/John/g");
    assert!(parsed.is_ok());
    let x = parsed.unwrap();
    assert_eq!(x.location, "mod.rs:");
    assert_eq!(x.find, "jon");
    assert_eq!(x.replace, "John");
    assert_eq!(x.global, true);

}
#[test]
fn parsing_location() {
    println!("{:?}", location::LocationParser::new().parse("..s/../../"));
    assert!(location::LocationParser::new().parse("../../../:").is_ok());
    assert!(location::LocationParser::new().parse("%").is_ok());
    assert!(location::LocationParser::new().parse("%:").is_err());
}