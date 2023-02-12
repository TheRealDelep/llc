use crate::file::file_writer::FileWriter;

use super::node::{CEmitter, Expression};

pub struct Data {
    pub type_identifier: Box<str>,
    pub identifier: Box<str>,
    pub value: Expression,
}

impl Data {
    pub(crate) fn emit<'a>(&self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer
            .append_str(&self.type_identifier)
            .append_space()
            .append_str(&self.identifier)
            .append_str(" = ");

        self.value.emit(writer);

        writer.append(&';')
    }
}
