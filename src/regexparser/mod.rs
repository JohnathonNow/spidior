pub mod ast;
lalrpop_mod!(pub reg, "/regexparser/reg.rs");
lalrpop_mod!(pub set, "/regexparser/set.rs");
lalrpop_mod!(pub query, "/regexparser/query.rs");
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