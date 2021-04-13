use std::error::Error;

use textbuffer::TextBuffer;

use crate::{editing::textbuffer, regexparser::ast::{Replace, Replacement}};
use crate::nfa::matcher::Match;
use super::matcher::find;

pub fn replace(input: &String, replacement: Replace) -> Result<String, Box<dyn Error>> {
    let matches = find(&input, replacement.clone().find);
    let mut tb = TextBuffer::new();
    let mut offset:i32 = 0;
    tb.add(input);
    for m in matches {
        let r = replace_to_string(&replacement.replace, &m, input);
        tb.replace((m.start() as i32 + offset) as usize, m.len(), &r)?;
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
    assert_eq!(replace(&"joejoe".into(), regex)?, "joejoe");

    let regex = regexparser::parse("%s/bob|joe|e*/bob/g")?;
    assert_eq!(replace(&"joejoe".into(), regex)?, "bobbob");

    let regex = regexparser::parse("%s/bob|joe|e*/jack/g")?;
    assert_eq!(replace(&"joee".into(), regex)?, "jackjack");

    let regex = regexparser::parse("%s/bob|joe|e*/o/g")?;
    assert_eq!(replace(&"joeejoe".into(), regex)?, "ooo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"joejoejoejo".into(), regex)?, "bobjo");

    let regex = regexparser::parse("%s/(joe)*/bob/g")?;
    assert_eq!(replace(&"eee".into(), regex)?, "eee");

    let regex = regexparser::parse("%s/jo*e/bob/g")?;
    assert_eq!(replace(&"jejoejooeej".into(), regex)?, "bobbobbobej");

    let regex = regexparser::parse("%s/jo+e/bob/g")?;
    assert_eq!(replace(&"jejoejooeej".into(), regex)?, "jebobbobej");

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(replace(&"-2607".into(), regex)?, "-2607");

    let regex = regexparser::parse("%s/[a-z]*/bob/g")?;
    assert_eq!(replace(&"-2e6f0z7a".into(), regex)?, "-2bob6bob0bob7bob");

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"joe".into(), regex)?, "joe");

    let regex = regexparser::parse("%s/[^a-z]*/bob/g")?;
    assert_eq!(replace(&"2607".into(), regex)?, "bob");
    Ok(())
}
#[test]
fn test_replace_backref() -> Result<(), Box<dyn std::error::Error>> {
    use crate::{regexparser};
    let regex = regexparser::parse("%s/(1)/\\1\\1/g")?;
    assert_eq!(replace(&"1".into(), regex)?, "11");
    Ok(())
}