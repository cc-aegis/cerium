use std::ops::Range;
use crate::parser::cerium_type::CeriumType;

pub struct Function {
    pub name: Qualifier,
    pub parameters: Vec<(String, CeriumType)>,
    pub return_type: Option<CeriumType>,
    pub body: Box<AST>,
}

pub struct Scope {
    pub instruction: Box<AST>,
    pub rest: Option<Box<AST>>,
}

struct Qualifier {
    pub names: Vec<String>,
}

struct TypeCast {
    pub value: Box<AST>,
    pub target_type: Box<CeriumType>,
}

pub struct AST {
    idx: Range<usize>,
    node: ASTNode,
}

pub enum ASTNode {
    Function(Function),
    Scope(Scope),
}