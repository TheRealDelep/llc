use crate::file::file_writer::FileWriter;

pub struct Data {
    pub type_identifier: Box<str>,
    pub identifier: Box<str>,
}

impl Data {
    pub(crate) fn emit<'a>(&self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer
            .append_str(&self.type_identifier)
            .append(&' ')
            .append_str(&self.identifier)
            .append(&';')
    }
}
