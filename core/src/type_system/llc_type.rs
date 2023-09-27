#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    String,
    Char,
    Float {signed: bool, size: usize},
    Integer {signed: bool, size: usize}
}