use std::hash::Hash;

pub struct Identifier {
    pub name: Box<str>,
    pub type_state: TypeState
}

pub enum TypeState {
    Unchecked,
    Ok(Type),
    Invalid
}

pub enum Type {
    Numeric,
    String, 
}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}