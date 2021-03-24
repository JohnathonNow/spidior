use std::error::Error;

use textbuffer::TextBuffer;

use crate::{editing::textbuffer, regexparser::ast::{Replace, Replacement}};

use super::matcher::find;

pub fn replace(input: &String, replacement: Replace) -> Result<String, Box<dyn Error>> {
    let matches = find(&input, replacement.clone().find);
    let mut tb = TextBuffer::new();
    let mut offset:i32 = 0;
    tb.add(input);
    for m in matches {
        let r = replace_to_string(&replacement.replace);
        tb.replace((m.start() as i32 + offset) as usize, m.len(), &r)?;
        offset += r.len() as i32 - m.len() as i32;
    }
    Ok(tb.consume())
}

fn replace_to_string(replacement: &Replacement) -> String {
    let mut ret = String::new();
    for ri in &replacement.replacements {
        match ri {
            crate::regexparser::ast::ReplaceItem::String(s) => {
                ret += &s;
            }
            crate::regexparser::ast::ReplaceItem::BackRef(_) => {}
        }
    }
    ret
}

#[test]
fn test_replace_to_string() -> Result<(), Box<dyn std::error::Error>> {
    let x = Replacement{
        replacements: vec![
            crate::regexparser::ast::ReplaceItem::String("hello".into()),
            crate::regexparser::ast::ReplaceItem::String(" ".into()),
            crate::regexparser::ast::ReplaceItem::String("world".into()),
            crate::regexparser::ast::ReplaceItem::String("!".into()),
        ],
    };
    assert_eq!(replace_to_string(&x), "hello world!");
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
    Ok(())
}