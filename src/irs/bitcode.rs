use std::collections::HashMap;

use crate::{
    common::{BinaryOperator, UnaryOperator},
    Array,
};

#[derive(Debug, Clone)]
pub struct Module {
    pub procedures: Array<SymbolId, Procedure>,
}

pub type SymbolId = u32;

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub blocks: Array<BlockId, Block>,
    pub values: HashMap<Value, ValueInfo>,
    pub local_count: u32,
}

pub type BlockId = u32;

#[derive(Debug, Clone)]
pub struct Block {
    pub instrs: Vec<Instr>,
}

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Const {
        dest: Value,
        val: i64,
    },
    Mov {
        dest: Value,
        src: Value,
    },
    BinaryOperation {
        dest: Value,
        lhs: Value,
        rhs: Value,
        operator: BinaryOperator,
    },
    UnaryOperation {
        dest: Value,
        operand: Value,
        operator: UnaryOperator,
    },
    Branch {
        cond: Value,
        then_block: BlockId,
        else_block: BlockId,
    },
    Jump(BlockId),
    Print(Value),
    Return(Value),
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Value {
    Local(u32),
    Temp(u32),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValueInfo {
    pub writes: u16,
}
