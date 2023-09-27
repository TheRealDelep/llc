use std::hash::Hash;

use crate::type_system::llc_type::Type;

pub struct Identifier {
    pub name: Box<str>,
    pub type_state: TypeState
}

pub enum TypeState {
    Unchecked,
    Ok(Type),
    Invalid
}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}