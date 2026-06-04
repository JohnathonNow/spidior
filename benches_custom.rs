use spidior::nfa::queryengine::QueryEngine;
use spidior::regexparser;
use spidior::nfa::matcher::find;
use spidior::languages::clike::Clike;
use std::time::Instant;

fn main() {
    let s = "a".repeat(10000) + "b";
    let regex = regexparser::parse("%s/a*b//g").unwrap().find;
    let mut qe = QueryEngine::build(&s, Box::new(Clike {}), Box::new(Clike {}));

    let t0 = Instant::now();
    for _ in 0..100 {
        let res = find(&mut qe, &s, regex.clone());
        assert_eq!(res.len(), 1);
    }
    println!("Found matches in {:?}", t0.elapsed());
}
