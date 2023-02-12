use crate::file::file_writer::FileWriter;

use super::node::{self, CEmitter};

pub struct Data {
    pub identifier: Box<str>,
    pub value: node::Expression,
}

impl Data {
    pub(crate) fn emit<'a>(&self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer.append_str(&self.identifier).append_str(&" = ");

        self.value.emit(writer);
        writer.append(&';')
    }
}
