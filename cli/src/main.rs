use std::env;

use ccg::{
    ast::{
        func_call,
        func_def::{self, FuncParam},
        node::{
            AstNode::{FuncDefinition, VarAssignment, VarDeclAssignment, VarDeclaration},
            Expression::{FunctionCall, Litteral, Variable},
        },
        var_assign, var_decl, var_declassign,
    },
    file::c_file::CFile,
};

use llc_core::parser::parser::FileAst;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = match args.get(1) {
        Some(p) => p,
        None => "src/test.llc"
    };

    let res = FileAst::new(path);

    for node in res.nodes {
        print!("{} ", node);
    }

    if !res.errors.is_empty() {
        eprintln!("Build failed: Errors happened while compiling the file {}.", res.file_name)
    }
    for r in res.errors.iter() {
        eprintln!("{0}", r)
    }
}

fn _test_ccg() {
    let decl = VarDeclaration(var_decl::Data {
        type_identifier: Box::from("int"),
        identifier: Box::from("i"),
    });

    let assign = VarAssignment(var_assign::Data {
        identifier: Box::from("i"),
        value: Litteral(Box::from("2")),
    });

    let declassign = VarDeclAssignment(var_declassign::Data {
        type_identifier: Box::from("int"),
        identifier: Box::from("j"),
        value: FunctionCall(func_call::Data {
            identifier: Box::from("printf"),
            params: Vec::from([Variable(Box::from("i"))]),
        }),
    });

    let param_lhs = FuncParam {
        type_identifier: Box::from("char*"),
        identifier: Box::from("lhs"),
    };

    let param_rhs = FuncParam {
        type_identifier: Box::from("int"),
        identifier: Box::from("rhs"),
    };

    let func = FuncDefinition(func_def::Data {
        identifier: Box::from("my_func"),
        type_identifier: Box::from("void"),
        params: Vec::from([param_lhs, param_rhs]),
        body: Vec::from([decl, assign, declassign]),
    });

    let file = CFile { nodes: vec![func] };

    file.print();
}
