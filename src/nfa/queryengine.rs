use crate::languages::parsing::{Function, Functions, Identifier, Identifiers};
pub struct QueryEngine {
    idents: Vec<Identifier>,
    functs: Vec<Function>,
    offset: usize,
}

impl QueryEngine {
    pub fn new() -> Self {
        Self {
            idents: vec![],
            functs: vec![],
            offset: 0,
        }
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn build(s: &String, i: Box<dyn Identifiers>, f: Box<dyn Functions>) -> Self {
        Self {
            idents: i.read_identifiers(s),
            functs: f.read_functions(s),
            offset: 0,
        }
    }

    pub fn query(&self, position: usize, query: &String) -> Option<usize> {
        let mut c = crate::regexparser::query::QueriesParser::new()
            .parse(query)
            .unwrap();
        let mut name = None;
        let mut kind = None;
        loop {
            match *c {
                crate::regexparser::ast::Queries::Query(x) => {
                    match *x {
                        crate::regexparser::ast::Query::Kv(k, v) if k == "type" => {
                            kind = Some(v);
                        }
                        crate::regexparser::ast::Query::Kv(k, v) if k == "name" => {
                            name = Some(v);
                        }
                        crate::regexparser::ast::Query::Kv(k, v) if k == "pos" => {
                            let mut s = v.split(":");
                            let (pos_str, len_str) = (s.next()?, s.next()?);
                            let (pos, len) = (pos_str.parse::<usize>().ok()?, len_str.parse::<usize>().ok()?);
                            if position + self.offset == pos {
                                return Some(len);
                            } else {
                                return None;
                            }
                        }
                        _ => {}
                    }
                    break;
                }
                crate::regexparser::ast::Queries::Queries(x, r) => {
                    match *x {
                        crate::regexparser::ast::Query::Kv(k, v) if k == "type" => {
                            kind = Some(v);
                        }
                        crate::regexparser::ast::Query::Kv(k, v) if k == "name" => {
                            name = Some(v);
                        }
                        crate::regexparser::ast::Query::Kv(k, v) if k == "pos" => {
                            let mut s = v.split(":");
                            let (pos_str, len_str) = (s.next()?, s.next()?);
                            let (pos, len) = (pos_str.parse::<usize>().ok()?, len_str.parse::<usize>().ok()?);
                            if position + self.offset == pos {
                                return Some(len);
                            } else {
                                return None;
                            }
                        }
                        _ => {}
                    }
                    c = r;
                }
            }
        }
        for ident in &self.idents {
            if match name {
                Some(ref y) => *y == ident.name,
                None => true,
            } && match kind {
                Some(ref y) => *y == ident.typ,
                None => true,
            } && position + self.offset == ident.start
            {
                return Some(ident.end - self.offset);
            }
        }
        None
    }
}
