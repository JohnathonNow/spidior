pub mod ast;
lalrpop_mod!(pub reg, "/regexparser/reg.rs");

#[test]
fn parsing() {
    assert!(reg::RegexParser::new().parse("a|b|c").is_ok());
    assert!(reg::RegexParser::new().parse("abc").is_ok());
    assert!(reg::RegexParser::new().parse("\\-").is_ok());
    assert!(reg::RegexParser::new().parse("[a-c]").is_ok());
    assert!(reg::RegexParser::new().parse("\\n").is_ok());
    assert!(reg::RegexParser::new().parse("[[name=x,type=int]]").is_ok());
    assert!(reg::RegexParser::new().parse("[[functions]]").is_ok());
    assert!(reg::RegexParser::new().parse("a|b|(").is_err());
    assert!(reg::RegexParser::new().parse("[[]]").is_err());
    assert!(reg::RegexParser::new().parse("[[bob]]").is_err());
}