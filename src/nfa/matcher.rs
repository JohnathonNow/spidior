use std::collections::HashSet;

use crate::nfa::{queryengine::QueryEngine};
use crate::nfa::Context;
use crate::nfa::Group;
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;
#[cfg(test)]
use crate::languages::clike::Clike;

use super::nfa_to_dfa;
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
    let o = build_nfa(regex);
    let (nfa, start, end) = nfa_to_dfa(&o.0, &o.1, &o.2);
    let mut ctx = Context::new(HashSet::new());
    eprintln!("NFA is `{}`", serde_json::to_string(&o.0).unwrap());
    eprintln!("DFA is `{}`", serde_json::to_string(&nfa).unwrap());

    let mut is = 0;
    while is < input.len() {
        ctx.reset();
        ctx.add_epsilons(vec![start].into_iter().collect(), &nfa);
        let mut new = None;
        let mut i = is;
        while i < input.len() && ctx.nodes.len() > 0 {
            let c = input.chars().nth(i).unwrap();
            qe.set_offset(is);
            i = is + ctx.step(&nfa, c, &qe);
            if nfa.accepts(&ctx.nodes) {
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
