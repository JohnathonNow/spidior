pub mod ast;
lalrpop_mod!(pub reg, "/regexparser/reg.rs");

#[test]
fn parsing() {
    //println!("{:?}", reg::RegexParser::new().parse("a|b|c"));
    //println!("{:?}", reg::RegexParser::new().parse("[a-z](bob)|joe"));
    assert!(reg::RegexParser::new().parse("a|b|c").is_ok());
    assert!(reg::RegexParser::new().parse("abc").is_ok());
    assert!(reg::RegexParser::new().parse("\\-").is_ok());
    assert!(reg::RegexParser::new().parse("[a-c]").is_ok());
    assert!(reg::RegexParser::new().parse("\\n").is_ok());
    assert!(reg::RegexParser::new().parse("a|b|(").is_err());
}