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
//!<set> 	::= 	<positive-set> | <negative-set>
//!<positive-set> 	::= 	"[" <set-items> "]"
//!<negative-set> 	::= 	"[^" <set-items> "]"
//!<set-items> 	::= 	<set-item> | <set-item> <set-items>
//!<set-item> 	::= 	<range> | <char>
//!<range> 	::= 	<char> "-" <char>
pub enum Union {
    O(Box<Union>, Box<Simple>),
}

pub enum Regex {
    Union(Box<Union>),
    Simple(Box<Simple>),
}

pub enum Simple {
    Concatenation(Box<Concatenation>),
    Basic(Box<Basic>),
}

pub enum Concatenation {
    O(Box<Simple>, Box<Basic>),
}

pub enum Basic {
    Star(Box<Star>),
    Plus(Box<Plus>),
    Elementary(Box<Elementary>),
}

pub enum Star {
    O(Box<Elementary>),
}

pub enum Plus {
    O(Box<Elementary>),
}

pub enum Elementary {
    Group(Box<Group>),
    Any(Box<Any>),
    Eos(Box<Eos>),
    Char(Box<Char>),
    Set(Box<Set>),
}

pub enum Group {
    O(Box<Regex>),
}

pub enum Any {
    O
}

pub enum Eos {
    O
}

pub enum Char {
    Char(char),
    Meta(char),
}

pub enum Set {
    Positive(Box<Positive>),
    Negative(Box<Negative>),
}

pub enum Positive {
    O(Box<Items>),
}

pub enum Negative {
    O(Box<Items>),
}

pub enum Items {
    Item(Box<Item>),
    Items(Box<Item>, Box<Items>),
}

pub enum Item {
    Range(Box<Range>),
    Char(Box<Char>),
}

pub enum Range {
    O(Box<Char>, Box<Char>),
}
