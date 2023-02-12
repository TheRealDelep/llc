use crate::file::file_writer::FileWriter;

use super::node::{AstNode, CEmitter};

pub struct Data {
    pub type_identifier: Box<str>,
    pub identifier: Box<str>,
    pub params: Vec<FuncParam>,
    pub body: Vec<AstNode>,
}

pub struct FuncParam {
    pub type_identifier: Box<str>,
    pub identifier: Box<str>,
}

impl CEmitter for Data {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer
            .append_str(&self.type_identifier)
            .append_space()
            .append_str(&self.identifier)
            .append(&'(');

        for (i, p) in self.params.iter().enumerate() {
            p.emit(writer);
            if i < self.params.len() - 1 {
                writer.append_str(", ");
            }
        }

        writer.append(&')').append_line();
        emit_body(&self.body, writer)
    }
}

impl CEmitter for FuncParam {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        writer
            .append_str(&self.type_identifier)
            .append_space()
            .append_str(&self.identifier)
    }
}

pub fn emit_body<'a>(nodes: &'a Vec<AstNode>, writer: &'a mut FileWriter) -> &'a mut FileWriter {
    writer.append(&'{').indent_right().append_line();

    for (i, s) in nodes.iter().enumerate() {
        s.emit(writer);
        if i < nodes.len() - 1 {
            writer.append_line();
        }
    }

    writer.indent_left().append_line().append(&'}')
}
