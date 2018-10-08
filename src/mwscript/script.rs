use super::funcs::{Func, Proc};

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
  pub name: String,
  pub instrs: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
  FloatVar(String),
  ShortVar(String),
  LongVar(String),
  SetTo(Option<String>, String, Expr),
  Proc(Option<String>, Proc, Vec<Arg>),
  If(Vec<(Expr, Vec<Instr>)>, Vec<Instr>),
  While(Expr, Vec<Instr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Long(i32),
  Float(f32),
  UnaryOp(UnaryOp, Box<Expr>),
  BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
  Func(Option<String>, Func, Vec<Arg>),
  Var(Option<String>, String),
  Brackets(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
  Id(String),
  Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  Less,
  LessOrEq,
  Greater,
  GreaterOrEq,
  Eq,
  NotEq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
  Plus,
  Minus,
}
