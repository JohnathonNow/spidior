use std::collections::HashSet;

use crate::nfa::{queryengine::QueryEngine};
use crate::nfa::Context;
use crate::nfa::Group;
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;
#[cfg(test)]
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

pub fn find(qe: &mut QueryEngine, input: &String, regex: Box<Regex>) -> Vec<Match> {
    let mut v = Vec::new();
    let (nfa, start, end) = build_nfa(regex);
    let mut ctx0 = Context::new(HashSet::new());
    ctx0.add_epsilons(vec![start].into_iter().collect(), &nfa);
    let mut is = 0;
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

#[ignore]
#[test]
fn test_find() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    let mut qe = QueryEngine::build(&"bob dole".to_string(), Box::new(Clike{}), Box::new(Clike{}));
    assert_eq!(find(&mut qe, &"bob dole".to_string(), regex).len(), 2); //matches bob and e
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    let mut qe = QueryEngine::build(&"bo".to_string(), Box::new(Clike{}), Box::new(Clike{}));
    assert_eq!(find(&mut qe, &"bo".to_string(), regex).len(), 0); //no match
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    let mut qe = QueryEngine::build(&"joee".to_string(), Box::new(Clike{}), Box::new(Clike{}));
    assert_eq!(find(&mut qe, &"joee".to_string(), regex).len(), 2); //"joe", "e"
    let regex = regexparser::parse("%s/(o*)o//g")?.find;
    let os = "ooooo";
    let mut qe = QueryEngine::build(&"ooooo".to_string(), Box::new(Clike{}), Box::new(Clike{}));
    let found = find(&mut qe, &os.to_string(), regex);
    assert_eq!(found.len(), 1); //entire string
    assert_eq!(found.get(0).unwrap().get_group(1, &os.to_string()), "oooo");
    Ok(())
}
