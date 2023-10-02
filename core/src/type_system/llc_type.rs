#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    String,
    Char,
    Float {signed: bool, size: usize},
    Integer {signed: bool, size: usize},
    Block {input: Option<Vec<Type>>, output: Option<Box<Type>>}
}