use std::collections::HashSet;

use crate::nfa::{NfaModel, queryengine::QueryEngine};
use crate::nfa::Context;
use crate::nfa::Group;
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;
use crate::languages::clike::Clike;
#[derive(Debug)]
pub struct Match {
    start: usize,
    len: usize,
    groups: Vec<Group>,
}

impl Match {
    pub fn new(start: usize, len: usize, _groups: Vec<Group>) -> Self {
        Self {
            start,
            len,
            groups: _groups,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_group(&self, i: usize, s: &String) -> String {
        if let Some(x) = self.groups.get(i) {
            s[x.start..x.start + x.len].to_string()
        } else {
            "".to_string()
        }
    }
}

pub fn find(input: &String, regex: Box<Regex>) -> Vec<Match> {
    let mut v = Vec::new();
    let (nfa, start, end) = build_nfa(regex);
    let mut ctx0 = Context::new(HashSet::new());
    ctx0.add_epsilons(vec![start].into_iter().collect(), &nfa);
    let mut is = 0;
    let mut qe = QueryEngine::build(input, Box::new(Clike{}), Box::new(Clike{}));
    while is < input.len() {
        let mut new = None;
        let mut ctx = ctx0.clone();
        let mut i = is;
        while i < input.len() {
            let c = input.chars().nth(i).unwrap();
            qe.set_offset(is);
            i = is + ctx.step(&nfa, c, &qe);
            if ctx.contains(&end) {
                new = Some(Match::new(is, i - is, ctx.groups.clone()));
            }
        }
        if let Some(x) = new {
            is += x.len - 1;
            v.push(x);
        }
        is += 1;
    }
    v
}


pub fn find_dfa(input: &String, regex: Box<Regex>) -> Vec<Match> {
    let mut v = Vec::new();
    let (nfa, start, end) = build_nfa(regex);

    let nfam = NfaModel::new(nfa, start, end);
    let nfam = nfam.to_dfa().unwrap();
    let (nfa, start, end) = (nfam.nfa, nfam.start, nfam.end);
    println!("{:?}", nfa);
    let mut ctx0 = Context::new(HashSet::new());
    ctx0.add_epsilons(vec![start].into_iter().collect(), &nfa);
    let mut is = 0;
    let mut qe = QueryEngine::build(input, Box::new(Clike{}), Box::new(Clike{}));
    while is < input.len() {
        let mut new = None;
        let mut ctx = ctx0.clone();
        let mut i = is;
        while i < input.len() {
            let c = input.chars().nth(i).unwrap();
            qe.set_offset(is);
            i = is + ctx.step(&nfa, c, &qe);
            if ctx.is_end(&nfa) {
                new = Some(Match::new(is, i - is, ctx.groups.clone()));
            }
        }
        if let Some(x) = new {
            is += x.len - 1;
            v.push(x);
        }
        is += 1;
    }
    v
}

#[ignore]
#[test]
fn test_find() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"bob dole".to_string(), regex).len(), 2); //matches bob and e
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"bo".to_string(), regex).len(), 0); //no match
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"joee".to_string(), regex).len(), 2); //"joe", "e"
    let regex = regexparser::parse("%s/(o*)o//g")?.find;
    let os = "ooooo";
    let found = find(&os.to_string(), regex);
    assert_eq!(found.len(), 1); //entire string
    assert_eq!(found.get(0).unwrap().get_group(1, &os.to_string()), "oooo");
    Ok(())
}

#[ignore]
#[test]
fn test_find_dfa() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find_dfa(&"bob dole".to_string(), regex).len(), 2); //matches bob and e
    let regex = regexparser::parse("%s/bob|joe|e+//g")?.find;
    assert_eq!(find_dfa(&"bo".to_string(), regex).len(), 0); //no match
    let regex = regexparser::parse("%s/bob|joe|e+//g")?.find;
    assert_eq!(find_dfa(&"joee".to_string(), regex).len(), 2); //"joe", "e"
    let regex = regexparser::parse("%s/(o*)o//g")?.find;
    let os = "ooooo";
    let found = find_dfa(&os.to_string(), regex);
    assert_eq!(found.len(), 1); //entire string
    assert_eq!(found.get(0).unwrap().get_group(1, &os.to_string()), "oooo");
    Ok(())
}
