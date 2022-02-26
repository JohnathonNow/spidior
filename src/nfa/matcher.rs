use std::collections::{HashSet, LinkedList};

#[cfg(test)]
use crate::languages::clike::Clike;
use crate::nfa::queryengine::QueryEngine;
use crate::nfa::Group;
use crate::nfa::{find_path, Context};
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;

use super::{nfa_to_dfa, Nfa, NodePointer};
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

fn path_to_matches(path: &Vec<crate::nfa::Path>, start: usize) -> Match {
    let mut groups = Vec::new();
    let mut len = 0;
    for p in path {
        match p {
            super::Path::Open(i) => {
                while i >= &groups.len() {
                    groups.push(Group {
                        start: start + len,
                        len: 0,
                    });
                }
            }
            super::Path::Close(i) => {
                while i >= &groups.len() {
                    groups.push(Group {
                        start: start + len,
                        len: 0,
                    });
                }
                groups[*i].len = len + start - groups[*i].start;
            }
            super::Path::Char => {
                len += 1;
            }
            super::Path::Query(x) => {
                len += x;
            }
        }
    }
    Match::new(start, len, groups)
}

pub fn find(qe: &mut QueryEngine, input: &String, regex: Box<Regex>) -> Vec<Match> {
    let mut v = Vec::new();
    let o = build_nfa(regex);
    let (nfa, start, end) = nfa_to_dfa(&o.0, &o.1, &o.2);
    let mut is = 0;
    while is <= input.len() {
        let mut path = Vec::new();
        //qe.set_offset(is);
        if find_path(qe, input, is, &nfa, start, &mut path) {
            let m = path_to_matches(&path, is);
            is += m.len;
            if m.len > 0 {
                v.push(m);
            } else {
                is += 1;
            }
        } else {
            is += 1;
        }
    }

    v
}

#[ignore]
#[test]
fn test_find() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    let mut qe = QueryEngine::build(
        &"bob dole".to_string(),
        Box::new(Clike {}),
        Box::new(Clike {}),
    );
    assert_eq!(find(&mut qe, &"bob dole".to_string(), regex).len(), 2); //matches bob and e
    let regex = regexparser::parse("%s/(bob)|(joe)|(a*)//g")?.find;
    let mut qe = QueryEngine::build(&"bo".to_string(), Box::new(Clike {}), Box::new(Clike {}));
    assert_eq!(find(&mut qe, &"bo".to_string(), regex).len(), 0); //no match
    let mut qe = QueryEngine::build(&"joejoe".to_string(), Box::new(Clike {}), Box::new(Clike {}));
    let regex = regexparser::parse("%s/(bob)|(joe)|(a*)//g")?.find;
    assert_eq!(find(&mut qe, &"joejoe".to_string(), regex).len(), 2); //two matches
    let regex = regexparser::parse("%s/bob|joe|e*//g")?.find;
    let mut qe = QueryEngine::build(&"joee".to_string(), Box::new(Clike {}), Box::new(Clike {}));
    assert_eq!(find(&mut qe, &"joee".to_string(), regex).len(), 2); //"joe", "e"
    let regex = regexparser::parse("%s/(o*)o//g")?.find;
    let os = "ooooo";
    let mut qe = QueryEngine::build(&"ooooo".to_string(), Box::new(Clike {}), Box::new(Clike {}));
    let found = find(&mut qe, &os.to_string(), regex);
    assert_eq!(found.len(), 1); //entire string
    assert_eq!(found.get(0).unwrap().get_group(1, &os.to_string()), "oooo");
    Ok(())
}