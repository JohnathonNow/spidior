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
#[derive(Debug)]
pub enum Union {
    O(Box<Regex>, Box<Simple>),
}

#[derive(Debug)]
pub enum Regex {
    Union(Box<Union>),
    Simple(Box<Simple>),
}

#[derive(Debug)]
pub enum Simple {
    Concatenation(Box<Concatenation>),
    Basic(Box<Basic>),
}

#[derive(Debug)]
pub enum Concatenation {
    O(Box<Simple>, Box<Basic>),
}

#[derive(Debug)]
pub enum Basic {
    Star(Box<Star>),
    Plus(Box<Plus>),
    Elementary(Box<Elementary>),
}

#[derive(Debug)]
pub enum Star {
    O(Box<Elementary>),
}

#[derive(Debug)]
pub enum Plus {
    O(Box<Elementary>),
}

#[derive(Debug)]
pub enum Elementary {
    Group(Box<Group>),
    Any(Box<Any>),
    Eos(Box<Eos>),
    Char(Box<Char>),
    Set(Box<Set>),
}

#[derive(Debug)]
pub enum Group {
    O(Box<Regex>),
}

#[derive(Debug)]
pub enum Any {
    O
}

#[derive(Debug)]
pub enum Eos {
    O
}

#[derive(Debug)]
pub enum Char {
    Char(char),
    Meta(char),
}

#[derive(Debug)]
pub enum Set {
    Positive(Box<Positive>),
    Negative(Box<Negative>),
    QuerySet(Box<QuerySet>,)
}

#[derive(Debug)]
pub enum Positive {
    O(Box<Items>),
}

#[derive(Debug)]
pub enum Negative {
    O(Box<Items>),
}

#[derive(Debug)]
pub enum QuerySet {
    O(Box<Items>),
}

#[derive(Debug)]
pub enum Items {
    Item(Box<Item>),
    Items(Box<Item>, Box<Items>),
}

#[derive(Debug)]
pub enum Item {
    Range(Box<Range>),
    Char(Box<Char>),
}

#[derive(Debug)]
pub enum Range {
    O(Box<Char>, Box<Char>),
}

#[derive(Debug)]
pub enum Queries {
    Query(Box<Query>),
    Queries(Box<Query>, Box<Queries>),
}

#[derive(Debug)]
pub enum Query {
    Kv(String, String),
    Fun
}
#[derive(Debug)]
pub enum Location {
    Path(String),
    All
}

#[derive(Debug)]
pub enum ReplaceItem {
    String(String),
    BackRef(i32),
}

#[derive(Debug)]
pub struct Replace {
    pub find: Box<Regex>,
    pub replace: Box<Replacement>,
    pub global: bool,
    pub location: Box<Location>
}

#[derive(Debug)]
pub struct ReplaceUnparsed {
    pub find: String,
    pub replace: String,
    pub location: String,
    pub global: bool,
}


#[derive(Debug)]
pub struct Replacement {
    pub replacements: Vec<ReplaceItem>
}
