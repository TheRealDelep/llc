use crate::common::position::FileSpan;

pub struct TypeError {
    pub position: FileSpan,
    pub reason: Box<str>
}