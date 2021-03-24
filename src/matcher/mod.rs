use crate::nfa::Context;
use crate::regex2nfa::build_nfa;
use crate::regexparser::ast::Regex;
pub struct Group {
    _start: usize,
    _end: usize,
}
pub struct Match {
    _start: usize,
    _end: usize,
    _groups: Vec<Group>,
}

impl Match {
    pub fn new(_start: usize, _end: usize, _groups: Vec<Group>) -> Self {
        Self {
            _start,
            _end,
            _groups,
        }
    }
}

pub fn find(input: &String, regex: Box<Regex>) -> Vec<Match> {
    let mut v = Vec::new();
    let (nfa, start, end) = build_nfa(regex);
    let ctx0 = Context::add_epsilons(vec![start].into_iter().collect(), &nfa);
    let mut ie = 0;
    for is in 0..input.len() {
        let mut ctx = ctx0.clone();
        for c in input.chars().skip(is) {
            ie += 1;
            ctx = ctx.step(&nfa, c);
            if ctx.contains(&end) {
                v.push(Match::new(is, ie, Vec::new()));
            }
        }
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
    assert_eq!(find(&"joee".to_string(), regex).len(), 4); //"joe", "e" (first), "ee", "e" (last)
    Ok(())
}
