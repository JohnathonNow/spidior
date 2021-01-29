use crate::parsing::{Function, Functions};

pub struct Java {}

enum FunctionFsm {
    NAME,
    SPACE,
    BRACE,
    NONE,
    PARENS(i32),
}

impl Functions for Java {
    fn read_functions(text: String) -> Vec<Function> {
        let mut s = FunctionFsm::NONE;
        let mut start = 0;
        let mut end = 0;
        let mut v = Vec::new();
        for (i, c) in text.chars().enumerate() {
            match s {
                FunctionFsm::NONE => {
                    if c.is_alphanumeric() {
                        s = FunctionFsm::NAME;
                        start = i;
                    }
                },
                FunctionFsm::NAME => {
                    if c == '(' {
                        s = FunctionFsm::PARENS(1);
                        end = i;
                    } else if !c.is_alphanumeric() {
                        s = FunctionFsm::NONE;
                    }
                },
                FunctionFsm::PARENS(j) => {
                    if c == '(' {
                        s = FunctionFsm::PARENS(j+1);
                    } else if c == ')' {
                        s = FunctionFsm::PARENS(j-1);
                    } else if c.is_whitespace() && j == 0 {
                        s = FunctionFsm::SPACE;
                    } else if c == '{' && j == 0 {
                        s = FunctionFsm::BRACE;
                    }
                },
                FunctionFsm::SPACE => {
                    if c.is_alphanumeric() {
                        s = FunctionFsm::NAME;
                        start = i;
                    } else if c == '{' {
                        s = FunctionFsm::BRACE;
                    } else if !c.is_whitespace() {
                        s = FunctionFsm::NONE;
                    }
                },
                FunctionFsm::BRACE => {
                    v.push(Function{ name: text[start..end].to_string() });
                    s = FunctionFsm::NONE;
                },
            }
        }
        v
    }
}

#[test]
fn functions() {
    let expected = "[Function { name: \"LightningOvercharge\" }, Function { name: \"getAction\" }, Function { name: \"onSpawn\" }, Function { name: \"getPassiveAction\" }, Function { name: \"getCost\" }, Function { name: \"getName\" }, Function { name: \"getTip\" }, Function { name: \"getActionNetwork\" }]";
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test/functions.java");
    let text = std::fs::read_to_string(d).unwrap();
    let result = format!("{:?}", Java::read_functions(text));
    assert_eq!(result, expected);
}
