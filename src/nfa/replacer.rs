use std::error::Error;

use textbuffer::TextBuffer;

use super::{matcher::find, queryengine::QueryEngine};
use crate::nfa::matcher::Match;
use crate::{
    editing::textbuffer,
    languages::clike::Clike,
    regexparser::ast::{Replace, Replacement},
};

pub type Acceptor = fn(&str, &str) -> bool;

pub fn replace(
    path_name: &String,
    input: &String,
    replacement: Replace,
    acceptor: Acceptor,
) -> Result<(String, bool), Box<dyn Error>> {
    let mut qe = QueryEngine::build(input, Box::new(Clike {}), Box::new(Clike {}));
    let matches = find(&mut qe, &input, replacement.clone().find);
    let mut tb = TextBuffer::new();
    let mut offset: i32 = 0;
    let mut changed = false;
    tb.add(input);
    for m in matches {
        let r = replace_to_string(&replacement.replace, &m, input);
        let start = (m.start() as i32 + offset) as usize;
        let to_replace = tb.get(start, m.len())?;

        let withinfunction = replacement.location.check(&input, start, path_name, &mut qe);

        if withinfunction && acceptor(&to_replace, &r) {
            tb.replace(start, m.len(), &r)?;
            offset += r.len() as i32 - m.len() as i32;
            changed = true;
        }
    }
    Ok((tb.consume(), changed))
}

fn replace_to_string(replacement: &Replacement, m: &Match, s: &String) -> String {
    let mut ret = String::new();
    for ri in &replacement.replacements {
        match ri {
            crate::regexparser::ast::ReplaceItem::String(s) => {
                ret += &s;
            }
            crate::regexparser::ast::ReplaceItem::BackRef(x) => {
                ret += &m.get_group(*x as usize, s);
            }
        }
    }
    ret
}

#[test]
fn test_replace_to_string() -> Result<(), Box<dyn std::error::Error>> {
    let m = Match::new(0, 0, vec![]);
    let x = Replacement {
        replacements: vec![
            crate::regexparser::ast::ReplaceItem::String("hello".into()),
            crate::regexparser::ast::ReplaceItem::String(" ".into()),
            crate::regexparser::ast::ReplaceItem::String("world".into()),
            crate::regexparser::ast::ReplaceItem::String("!".into()),
        ],
    };
    assert_eq!(replace_to_string(&x, &m, &"".to_string()), "hello world!");
    Ok(())
}

#[test]
fn test_replace() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/bill/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoe".into(), regex, |_, _| true)?.0, "joejoe");

    let regex = regexparser::parse("%s/(joe)|(bob)|(a*)/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoe".into(), regex, |_, _| true)?.0, "bobbob");

    let regex = regexparser::parse("%s/bob|joe|e*/jack/g")?;
    assert_eq!(replace(&"".into(), &"joee".into(), regex, |_, _| true)?.0, "jackjack");

    let regex = regexparser::parse("%s/bob|joe|e*/o/g")?;
    assert_eq!(replace(&"".into(), &"joeejoe".into(), regex, |_, _| true)?.0, "ooo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoejoejo".into(), regex, |_, _| true)?.0, "bobjo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"".into(), &"eee".into(), regex, |_, _| true)?.0, "eee");

    let regex = regexparser::parse("%s/jo*e/bob/g")?;
    assert_eq!(
        replace(&"".into(), &"jejoejooeej".into(), regex, |_, _| true)?.0,
        "bobbobbobej"
    );

    let regex = regexparser::parse("%s/jo+e/bob/g")?;
    assert_eq!(
        replace(&"".into(), &"jejoejooeej".into(), regex, |_, _| true)?.0,
        "jebobbobej"
    );

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(replace(&"".into(), &"-2607".into(), regex, |_, _| true)?.0, "-2607");

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(
        replace(&"".into(), &"-2e6f0z7a".into(), regex, |_, _| true)?.0,
        "-2bob6bob0bob7bob"
    );

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"".into(), &"joe".into(), regex, |_, _| true)?.0, "joe");

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"".into(), &"2607".into(), regex, |_, _| true)?.0, "bob");
    Ok(())
}
#[test]
fn test_replace_backref() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/(1)/\\1\\1/g")?;
    assert_eq!(replace(&"".into(), &"1".into(), regex, |_, _| true)?.0, "11");
    Ok(())
}

#[test]
fn test_pos() -> Result<(), Box<dyn std::error::Error>> {
    use crate::regexparser;
    let regex = regexparser::parse("%s/[[pos=0:3]]/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoe".into(), regex, |_, _| true)?.0, "bobjoe");
    let regex = regexparser::parse("%s/[[pos=1:3]]/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoe".into(), regex, |_, _| true)?.0, "jboboe");
    let regex = regexparser::parse("%s/[[pos=2:1]]joe/bob/g")?;
    assert_eq!(replace(&"".into(), &"joejoe".into(), regex, |_, _| true)?.0, "jobob");
    Ok(())
}
