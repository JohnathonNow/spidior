use spidior::nfa::queryengine::QueryEngine;
use spidior::regexparser;
use spidior::nfa::matcher::find;
use spidior::languages::clike::Clike;
use spidior::regex2nfa::build_nfa;
use spidior::nfa::nfa_to_dfa;
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let regex = regexparser::parse("%s/a*b//g").unwrap().find;
    let (nfa, start, end) = build_nfa(regex);
    let (dfa, s, e) = nfa_to_dfa(&nfa, &start, &end);
    println!("NFA to DFA took {:?}", t0.elapsed());

    let regex2 = regexparser::parse("%s/a*b|c*d|e*f|g*h//g").unwrap().find;
    let (nfa2, start2, end2) = build_nfa(regex2);
    let t1 = Instant::now();
    let (dfa2, s2, e2) = nfa_to_dfa(&nfa2, &start2, &end2);
    println!("NFA to DFA for larger regex took {:?}", t1.elapsed());
}
