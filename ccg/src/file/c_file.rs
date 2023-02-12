use crate::{
    ast::node::{AstNode, CEmitter},
    file::file_writer,
};

pub struct CFile {
    pub nodes: Vec<AstNode>,
}

impl CFile {
    pub fn print(&self) {
        let mut writer = file_writer::FileWriter::new();

        for n in self.nodes.as_slice() {
            n.emit(&mut writer);
            writer.append_line();
        }

        println!("{}", writer.get_content());
    }
}
