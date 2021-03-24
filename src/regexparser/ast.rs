//! Contains nodes for the AST of regular expressions, with the following rules
//!<RE> 	::= 	<union> | <simple-RE>
//!<union> 	::=	<RE> "|" <simple-RE>
//!<simple-RE> 	::= 	<concatenation> | <basic-RE>
//!<concatenation> 	::=	<simple-RE> <basic-RE>
//!<basic-RE> 	::=	<star> | <plus> | <elementary-RE>
//!<star> 	::=	<elementary-RE> "*"
//!<plus> 	::=	<elementary-RE> "+"
//!<elementary-RE> 	::=	<group> | <any> | <eos> | <char> | <set>
//!<group> 	::= 	"(" <RE> ")"
//!<any> 	::= 	"."
//!<eos> 	::= 	"$"
//!<char> 	::= 	any non metacharacter | "\" metacharacter
//!<set> 	::= 	<positive-set> | <negative-set> | <query-set>
//!<positive-set> 	::= 	"[" <set-items> "]"
//!<negative-set> 	::= 	"[^" <set-items> "]"
//!<query-set>      ::=     "[[" <query-items> "]]"
//!<set-items> 	::= 	<set-item> | <set-item> <set-items>
//!<set-item> 	::= 	<range> | <char>
//!<query-items>        ::=         <query> | <query> <query-items>
//!<range> 	::= 	<char> "-" <char>
#[derive(Debug, Clone)]
pub enum Union {
    O(Box<Regex>, Box<Simple>),
}

#[derive(Debug, Clone)]
pub enum Regex {
    Union(Box<Union>),
    Simple(Box<Simple>),
}

#[derive(Debug, Clone)]
pub enum Simple {
    Concatenation(Box<Concatenation>),
    Basic(Box<Basic>),
}

#[derive(Debug, Clone)]
pub enum Concatenation {
    O(Box<Simple>, Box<Basic>),
}

#[derive(Debug, Clone)]
pub enum Basic {
    Star(Box<Star>),
    Plus(Box<Plus>),
    Elementary(Box<Elementary>),
}

#[derive(Debug, Clone)]
pub enum Star {
    O(Box<Elementary>),
}

#[derive(Debug, Clone)]
pub enum Plus {
    O(Box<Elementary>),
}

#[derive(Debug, Clone)]
pub enum Elementary {
    Group(Box<Group>),
    Any(Box<Any>),
    Eos(Box<Eos>),
    Char(Box<Char>),
    Set(Box<Set>),
}

#[derive(Debug, Clone)]
pub enum Group {
    O(Box<Regex>),
}

#[derive(Debug, Clone)]
pub enum Any {
    O
}

#[derive(Debug, Clone)]
pub enum Eos {
    O
}

#[derive(Debug, Clone)]
pub enum Char {
    Char(char),
    Meta(char),
}

#[derive(Debug, Clone)]
pub enum Set {
    Positive(Box<Positive>),
    Negative(Box<Negative>),
    QuerySet(Box<QuerySet>,)
}

#[derive(Debug, Clone)]
pub enum Positive {
    O(Box<Items>),
}

#[derive(Debug, Clone)]
pub enum Negative {
    O(Box<Items>),
}

#[derive(Debug, Clone)]
pub enum QuerySet {
    O(Box<Items>),
}

#[derive(Debug, Clone)]
pub enum Items {
    Item(Box<Item>),
    Items(Box<Item>, Box<Items>),
}

#[derive(Debug, Clone)]
pub enum Item {
    Range(Box<Range>),
    Char(Box<Char>),
}

#[derive(Debug, Clone)]
pub enum Range {
    O(Box<Char>, Box<Char>),
}

#[derive(Debug, Clone)]
pub enum Queries {
    Query(Box<Query>),
    Queries(Box<Query>, Box<Queries>),
}

#[derive(Debug, Clone)]
pub enum Query {
    Kv(String, String),
    Fun
}
#[derive(Debug, Clone)]
pub enum Location {
    Path(String),
    All
}

#[derive(Debug, Clone)]
pub enum ReplaceItem {
    String(String),
    BackRef(i32),
}

#[derive(Debug, Clone)]
pub struct Replace {
    pub find: Box<Regex>,
    pub replace: Box<Replacement>,
    pub global: bool,
    pub location: Box<Location>
}

#[derive(Debug, Clone)]
pub struct ReplaceUnparsed {
    pub find: String,
    pub replace: String,
    pub location: String,
    pub global: bool,
}


#[derive(Debug, Clone)]
pub struct Replacement {
    pub replacements: Vec<ReplaceItem>
}
