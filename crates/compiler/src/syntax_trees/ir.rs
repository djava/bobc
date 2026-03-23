use super::shared::*;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AtomValue {
    Constant(Value),
    Variable(Identifier),
    GlobalSymbol(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub value: AtomValue,
    pub size: usize,
}

impl Atom {
    pub fn new_constant(value: Value, size: usize) -> Self {
        Self {
            value: AtomValue::Constant(value),
            size,
        }
    }

    pub fn new_variable(id: Identifier, size: usize) -> Self {
        Self {
            value: AtomValue::Variable(id),
            size,
        }
    }

    pub fn new_global(id: Identifier, size: usize) -> Self {
        Self {
            value: AtomValue::GlobalSymbol(id),
            size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Atom),
    UnaryOp(UnaryOperator, Atom),
    BinaryOp(Atom, BinaryOperator, Atom),
    Call(Atom, Vec<Atom>),
    Allocate(usize, ValueType),
    TupleSubscript(Atom, i64),
    ArrayUncheckedSubscript(Atom, i64, usize),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Assign(SizedAssignDest<()>, Expr),
    Return(Atom),
    Goto(Identifier),
    If(Expr, Identifier, Identifier),
    TailCall(Atom, Vec<Atom>),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

pub type BlockMap = IndexMap<Identifier, Block>;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Identifier,
    pub params: IndexMap<Identifier, ValueType>,
    pub return_type: ValueType,
    pub blocks: BlockMap,
    pub entry_block: Identifier,
    pub exit_block: Identifier,
    pub types: TypeEnv,
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<Function>,
}
