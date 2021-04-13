use std::collections::HashSet;

use crate::nfa::Context;
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;
use crate::nfa::Group;
pub struct Match {
    start: usize,
    len: usize,
    _groups: Vec<Group>,
}

impl Match {
    pub fn new(start: usize, len: usize, _groups: Vec<Group>) -> Self {
        Self {
            start,
            len,
            _groups,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get_group(&self, i: usize, s: &String) -> String {
        if let Some(x) = self._groups.get(i) {
            s[x.start..x.start+x.len].to_string()
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
    while is < input.len() {
        let mut new = None;
        let mut ctx = ctx0.clone();
        let mut il = 0;
        for c in input.chars().skip(is) {
            il += 1;
            ctx.step(&nfa, c);
            if ctx.contains(&end) {
                new = Some(Match::new(is, il, ctx.groups.clone()));
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

#[test]
fn test_find() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"bob dole".to_string(), regex).len(), 2); //matches bob and e
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"bo".to_string(), regex).len(), 0); //no match
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    assert_eq!(find(&"joee".to_string(), regex).len(), 2); //"joe", "e"
    Ok(())
}
