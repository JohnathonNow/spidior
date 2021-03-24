//! This module is for building an `nfa::Nfa` from a
//! a `regexparser::ast::Regex`
use crate::nfa::NodePointer;

use super::nfa::Nfa;
use super::regexparser::ast::*;

pub fn build_nfa(r: Box<Regex>) -> Nfa {
    let mut nfa = Nfa::new(Vec::new());
    do_regex(r, &mut nfa);
    nfa
}

fn do_regex(r: Box<Regex>, nfa: &mut Nfa) -> NodePointer {
    match *r {
        Regex::Union(r) => do_union(r, nfa),
        Regex::Simple(r) => do_simple(r, nfa),
    }
}

fn do_union(r: Box<Union>, nfa: &mut Nfa) -> NodePointer {
    let Union::O(x, y) = *r;
    let a = do_regex(x, nfa);
    let b = do_simple(y, nfa);
    let n = nfa.new_node();
    nfa.add_transition_epsilon(&n, &a).unwrap();
    nfa.add_transition_epsilon(&n, &b).unwrap();
    n
}

fn do_simple(r: Box<Simple>, nfa: &mut Nfa) -> NodePointer {
    match *r {
        Simple::Concatenation(r) => do_concat(r, nfa),
        Simple::Basic(r) => do_basic(r, nfa),
    }
}

fn do_basic(r: Box<Basic>, nfa: &mut Nfa) -> NodePointer {
    match *r {
        Basic::Star(r) => do_star(r, nfa),
        Basic::Plus(r) => do_plus(r, nfa),
        Basic::Elementary(r) => do_elem(r, nfa),
    }
}

fn do_concat(r: Box<Concatenation>, nfa: &mut Nfa) -> NodePointer {
    let Concatenation::O(x, y) = *r;
    let src = do_simple(x, nfa);
    let dst = do_basic(y, nfa);
    nfa.add_transition_epsilon(&src, &dst).unwrap();
    src
}

fn do_elem(r: Box<Elementary>, nfa: &mut Nfa) -> NodePointer {
    match *r {
        Elementary::Group(r) => unimplemented!(),
        Elementary::Any(r) => unimplemented!(),
        Elementary::Eos(r) => unimplemented!(),
        Elementary::Char(r) => do_char(r, nfa),
        Elementary::Set(r) => unimplemented!(),
    }
}

fn do_star(r: Box<Star>, nfa: &mut Nfa) -> NodePointer {
    let Star::O(r) = *r;
    let src = do_elem(r, nfa);
    nfa.add_transition_epsilon(&src,&src).unwrap();
    src
}

fn do_plus(r: Box<Plus>, nfa: &mut Nfa) -> NodePointer {
    unimplemented!();
}

fn do_char(r: Box<Char>, nfa: &mut Nfa) -> NodePointer {
    let src = nfa.new_node();
    let dst = nfa.new_node();
    let c = match *r {
        Char::Char(c) => c,
        Char::Meta(c) => c,
    };
    nfa.add_transition_alpha(&src, &dst, c).unwrap();
    src
}
