use crate::file::file_writer::FileWriter;

use super::node::{CEmitter, Expression};

pub struct Data {
    pub identifier: Box<str>,
    pub params: Vec<Expression>,
}

impl CEmitter for Data {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer.append_str(&self.identifier).append(&'(');

        for (i, p) in self.params.iter().enumerate() {
            p.emit(writer);
            if i < self.params.len() - 1 {
                writer.append_str(", ");
            }
        }

        writer.append(&')')
    }
}
