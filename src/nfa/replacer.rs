use std::error::Error;

use textbuffer::TextBuffer;

use crate::{editing::textbuffer, regexparser::ast::{Replace, Replacement}};
use crate::nfa::matcher::Match;
use super::matcher::find;

pub type Acceptor = fn(&str, &str) -> bool;

pub fn replace(input: &String, replacement: Replace, acceptor: Acceptor) -> Result<String, Box<dyn Error>> {
    let matches = find(&input, replacement.clone().find);
    let mut tb = TextBuffer::new();
    let mut offset:i32 = 0;
    tb.add(input);
    for m in matches {
        let r = replace_to_string(&replacement.replace, &m, input);
        let start = (m.start() as i32 + offset) as usize;
        let to_replace = tb.get(start, m.len())?;
        if acceptor(&to_replace, &r) {
            tb.replace(start, m.len(), &r)?;
        }
        offset += r.len() as i32 - m.len() as i32;
    }
    Ok(tb.consume())
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
    let x = Replacement{
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
    use crate::{regexparser};
    let regex = regexparser::parse("%s/bill/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex, |x, y| true)?, "joejoe");

    let regex = regexparser::parse("%s/bob|joe|e*/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex, |x, y| true)?, "bobbob");

    let regex = regexparser::parse("%s/bob|joe|e*/jack/g")?;
    assert_eq!(replace(&"joee".into(), regex, |x, y| true)?, "jackjack");

    let regex = regexparser::parse("%s/bob|joe|e*/o/g")?;
    assert_eq!(replace(&"joeejoe".into(), regex, |x, y| true)?, "ooo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"joejoejoejo".into(), regex, |x, y| true)?, "bobjo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"eee".into(), regex, |x, y| true)?, "eee");

    let regex = regexparser::parse("%s/jo*e/bob/g")?;
    assert_eq!(replace(&"jejoejooeej".into(), regex, |x, y| true)?, "bobbobbobej");

    let regex = regexparser::parse("%s/jo+e/bob/g")?;
    assert_eq!(replace(&"jejoejooeej".into(), regex, |x, y| true)?, "jebobbobej");

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(replace(&"-2607".into(), regex, |x, y| true)?, "-2607");

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(replace(&"-2e6f0z7a".into(), regex, |x, y| true)?, "-2bob6bob0bob7bob");

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"joe".into(), regex, |x, y| true)?, "joe");

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"2607".into(), regex, |x, y| true)?, "bob");
    Ok(())
}
#[test]
fn test_replace_backref() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{regexparser};
    let regex = regexparser::parse("%s/(1)/\\1\\1/g")?;
    assert_eq!(replace(&"1".into(), regex, |x, y| true)?, "11");
    Ok(())
}


#[test]
fn test_pos() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{regexparser};
    let regex = regexparser::parse("%s/[[pos=0:3]]/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex, |x, y| true)?, "bobjoe");
    let regex = regexparser::parse("%s/[[pos=1:3]]/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex, |x, y| true)?, "jboboe");
    let regex = regexparser::parse("%s/[[pos=2:1]]joe/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex, |x, y| true)?, "jobob");
    Ok(())
}