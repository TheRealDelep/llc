// use crate::{
//     common::identifier::{TypeState, Identifier},
//     parser::{
//         ast_node::{AstNodeKind, NodeParent},
//         expression::Expression,
//         parser::FileAst,
//         statement::Statement,
//     },
// };
// 
// use super::llc_type::Type;
// 
// pub fn check_file(file_ast: &mut FileAst) {
//     let mut index = 0;
// 
//     loop {
//         if  index >= file_ast.root_nodes.len() -1{
//             break; 
//         }
// 
//         let node_index = file_ast.root_nodes[index];
//         let node = &mut file_ast.nodes[node_index];
// 
//         node.parent = NodeParent::Root;
//         match node.kind {
//             AstNodeKind::Statement(Statement::Declaration { ident_index }) => {
//                 check_declaration(file_ast, ident_index, node_index);
//             }
//             _ => panic!("Expected a declaration at file root."),
//         }
// 
//         index += 1;
//     }
// }
// 
// fn check_declaration(file_ast: &mut FileAst, ident_index: usize, decl_index: usize) {
//     let exp_type = check_expression(file_ast, ident_index + 1, decl_index - 1);
//     let ident = match file_ast.nodes[ident_index].kind {
//         AstNodeKind::Expression(Expression::Identifier { index }) => {
//             &mut file_ast.identifiers[index]
//         }
//         _ => panic!("Expected an identifier"),
//     };
// 
//     if ident.type_state != TypeState::Unchecked {
//         return;
//     }
// 
//     ident.type_state = exp_type;
// }
// 
// fn check_expression(file_ast: &mut FileAst, begin_index: usize, exp_index: usize) -> TypeState {
//     let kind = &file_ast.nodes[exp_index].kind; 
//     match kind {
//         AstNodeKind::Expression(Expression::Identifier { index }) => {
//             check_identifier(file_ast, begin_index,*index) 
//         }
//         AstNodeKind::Expression(Expression::Literal(lit)) => TypeState::Ok(lit.value.llc_type.to_owned()),
//         AstNodeKind::Expression(Expression::Block) => TypeState::Ok(Type::Block {
//             input: None,
//             output: None,
//         }),
//         AstNodeKind::Expression(_) => todo!(),
//         _ => panic!("Expected an expression"),
//     }
// }
// 
// fn check_identifier(file_ast: &mut FileAst, ident_node_index: usize, ident_index: usize) -> TypeState {
//     let decl_index = match &file_ast.identifiers[ident_index] {
//         id @ Identifier {type_state: TypeState::Unchecked, ..} => id.declaration_idx,
//         id @ _ => return id.type_state.to_owned()
//     };
// 
//     check_declaration(file_ast, ident_node_index, decl_index);
//     file_ast.identifiers[ident_index].type_state.to_owned()
// }
