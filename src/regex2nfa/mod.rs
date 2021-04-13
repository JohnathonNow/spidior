//! This module is for building an `nfa::Nfa` from a
//! a `regexparser::ast::Regex`

use std::collections::HashSet;

use crate::{nfa::NodePointer, regexparser::parse_set};

use super::nfa::Nfa;
use super::regexparser::ast::*;

pub fn build_nfa(r: Box<Regex>) -> (Nfa, NodePointer, NodePointer) {
    let mut nfa = Nfa::new(Vec::new());
    let (s, d) = do_regex(r, &mut nfa);
    (nfa, s, d)
}

fn do_regex(r: Box<Regex>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    match *r {
        Regex::Union(r) => do_union(r, nfa),
        Regex::Simple(r) => do_simple(r, nfa),
    }
}

fn do_union(r: Box<Union>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Union::O(x, y) = *r;
    let a = do_regex(x, nfa);
    let b = do_simple(y, nfa);
    let s = nfa.new_node();
    let d = nfa.new_node();
    nfa.add_transition_epsilon(&s, &a.0).unwrap();
    nfa.add_transition_epsilon(&s, &b.0).unwrap();
    nfa.add_transition_epsilon(&a.1, &d).unwrap();
    nfa.add_transition_epsilon(&b.1, &d).unwrap();
    (s, d)
}

fn do_simple(r: Box<Simple>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    match *r {
        Simple::Concatenation(r) => do_concat(r, nfa),
        Simple::Basic(r) => do_basic(r, nfa),
    }
}

fn do_basic(r: Box<Basic>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    match *r {
        Basic::Star(r) => do_star(r, nfa),
        Basic::Plus(r) => do_plus(r, nfa),
        Basic::Elementary(r) => do_elem(r, nfa),
    }
}

fn do_concat(r: Box<Concatenation>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Concatenation::O(x, y) = *r;
    let (ls, ld) = do_simple(x, nfa);
    let (rs, rd) = do_basic(y, nfa);
    nfa.add_transition_epsilon(&ld, &rs).unwrap();
    (ls, rd)
}

fn do_elem(r: Box<Elementary>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    match *r {
        Elementary::Group(r) => do_group(r, nfa),
        Elementary::Any(_) => unimplemented!(),
        Elementary::Eos(_) => unimplemented!(),
        Elementary::Char(r) => do_char(r, nfa),
        Elementary::Set(r) => do_set(r, nfa),
    }
}

fn do_star(r: Box<Star>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Star::O(r) = *r;
    let (src, dst) = do_elem(r, nfa);
    nfa.add_transition_epsilon(&dst, &src).unwrap();
    nfa.add_transition_epsilon(&src, &dst).unwrap();

    (src, dst)
}

fn do_plus(r: Box<Plus>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Plus::O(r) = *r;
    let (src, dst) = do_elem(r, nfa);
    nfa.add_transition_epsilon(&dst, &src).unwrap();
    (src, dst)
}

fn do_char(r: Box<Char>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let src = nfa.new_node();
    let dst = nfa.new_node();
    let c = match *r {
        Char::Char(c) => c,
        Char::Meta(c) => c,
    };
    nfa.add_transition_alpha(&src, &dst, c).unwrap();
    (src, dst)
}

fn do_set(r: Box<Set>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    match *r {
        Set::Positive(r) => do_positive(r, nfa),
        Set::Negative(r) => do_negative(r, nfa),
        Set::QuerySet(r) => do_queryset(r, nfa),
    }
}

fn do_queryset(r: Box<QuerySet>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let QuerySet::O(r) = *r;
    let r = parse_set(get_string(r));
    let src = nfa.new_node();
    let dst = nfa.new_node();
    nfa.add_transition_queryset(&src, &dst, get_string(r)).unwrap();
    (src, dst)
}

fn do_negative(r: Box<Negative>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Negative::O(r) = *r;
    let r = parse_set(get_string(r));
    let src = nfa.new_node();
    let dst = nfa.new_node();
    nfa.add_transition_negativerange(&src, &dst, get_string(r)).unwrap();
    (src, dst)
}

fn do_positive(r: Box<Positive>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Positive::O(r) = *r;
    let r = parse_set(get_string(r));
    let src = nfa.new_node();
    let dst = nfa.new_node();
    nfa.add_transition_range(&src, &dst, get_string(r)).unwrap();
    (src, dst)
}

fn get_string(r: Box<Items>) -> String {
    let mut s = String::new();
    for item in get_items(r) {
        match *item {
            Item::Range(r) => {
                let Range::O(a, b) = *r;
                for c in get_char(a)..=get_char(b) {
                    s.push(c);
                }
            }
            Item::Char(c) => {
                s.push(get_char(c));
            }
        }
    }
    s
}

fn get_char(r: Box<Char>) -> char {
    match *r {
        Char::Char(c) => c,
        Char::Meta(c) => c,
    }
}

fn get_items(r: Box<Items>) -> Vec<Box<Item>> {
    let mut r = r;
    let mut v = Vec::new();
    loop {
        match *r {
            Items::Item(i) => {
                v.push(i);
                return v;
            }
            Items::Items(i, nr) => {
                v.push(i);
                r = nr;
            }
        }
    }
}


fn do_group(r: Box<Group>, nfa: &mut Nfa) -> (NodePointer, NodePointer) {
    let Group::O(r) = *r;
    let src = nfa.new_node();
    let dst = nfa.new_node();
    let x = do_regex(r, nfa);
    nfa.add_group(&src, &x.0, &x.1, &dst).unwrap();
    (src, dst)
}


#[test]
fn test_regex() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{nfa::Context, regexparser};

    let regex = regexparser::parse("%s/bob|joe|e*//g")?;
    let (nfa, start, end) = build_nfa(regex.find);
    let mut ctx = Context::new(HashSet::new());
    ctx.add_epsilons(vec![start].into_iter().collect(), &nfa);
    for c in "bob".chars() {
        ctx.step(&nfa, c);
    }
    assert!(ctx.contains(&end));
    let mut ctx = Context::new(HashSet::new());
    ctx.add_epsilons(vec![start].into_iter().collect(), &nfa);
    for c in "bobd".chars() {
        ctx.step(&nfa, c);
    }
    assert!(!ctx.contains(&end));
    let mut ctx = Context::new(HashSet::new());
    ctx.add_epsilons(vec![start].into_iter().collect(), &nfa);
    for c in "bo".chars() {
        ctx.step(&nfa, c);
    }
    assert!(!ctx.contains(&end));
    let mut ctx = Context::new(HashSet::new());
    ctx.add_epsilons(vec![start].into_iter().collect(), &nfa);
    for c in "eeeeeeeeee".chars() {
        ctx.step(&nfa, c);
    }
    assert!(ctx.contains(&end));
    Ok(())
}
