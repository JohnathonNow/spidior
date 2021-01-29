#[derive(Debug)]
pub struct Function {
    pub name: String,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub typ: String,
    pub start: usize,
    pub end: usize,
}

pub trait Functions {
    fn read_functions(text: String) -> Vec<Function>;
}

pub trait Identifiers {
    fn read_identifiers(text: String) -> Vec<Identifier>;
}
