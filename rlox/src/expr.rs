use crate::token::Token;
use std::boxed::Box;

pub enum LiteralType {
    Number(f32),
    String(String),
    Nil,
    Boolean(bool),
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralType),
    Unary(Token, Box<Expr>),
}
