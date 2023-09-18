use super::{ast_node::AstNode, syntax_error::SyntaxError};

pub(crate) struct ParserBuffer {
    pub(crate) nodes: Vec<AstNode>,
    pub(crate) errors: Vec<SyntaxError>,
    offset: usize
}

impl ParserBuffer {
    pub fn new() -> Self {
        ParserBuffer {
            nodes: vec![],
            errors: vec![],
            offset: 0
        }
    }

    pub fn push_node(&mut self, node: AstNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn push_error(&mut self, error: SyntaxError) {
        self.errors.push(error)
    }

    pub fn get(&self, id: usize) -> &AstNode {
        &self.nodes[id]
    }

    pub fn get_node_id(&self, index: usize) -> usize {
        index 
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}
