use super::func_call;
use super::func_def;
use super::var_assign;
use super::var_decl;
use super::var_declassign;
use crate::file::file_writer::FileWriter;

pub(crate) trait CEmitter {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &'a mut FileWriter;
}

pub enum AstNode {
    VarDeclaration(var_decl::Data),
    VarAssignment(var_assign::Data),
    VarDeclAssignment(var_declassign::Data),
    FuncDefinition(func_def::Data),
}

pub enum Expression {
    Variable(Box<str>),
    Litteral(Box<str>),
    FunctionCall(func_call::Data),
}

impl CEmitter for AstNode {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &mut FileWriter {
        match self {
            AstNode::VarAssignment(data) => data.emit(writer),
            AstNode::VarDeclaration(data) => data.emit(writer),
            AstNode::VarDeclAssignment(data) => data.emit(writer),
            AstNode::FuncDefinition(data) => data.emit(writer),
        }
    }
}

impl CEmitter for Expression {
    fn emit<'a>(&'a self, writer: &'a mut FileWriter) -> &'a mut FileWriter {
        match self {
            Expression::Variable(id) => writer.append_str(id),
            Expression::Litteral(value) => writer.append_str(value),
            Expression::FunctionCall(data) => data.emit(writer),
        }
    }
}
