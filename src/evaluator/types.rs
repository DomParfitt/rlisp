use std::fmt::{Display, Formatter, Result};
use crate::parser::grammar::Expr;
use crate::evaluator::scope::Scope;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Type {
    Err(String),
    //    Num(Num), //TODO: numerical type with more specific subtypings
    Int(isize),
    Bool(bool),
    Str(String),
    Seq(Vec<Type>),
    Fn(Vec<String>, Expr, Rc<RefCell<Scope>>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Type::Err(ref err) => write!(f, "{}", err),
            Type::Int(ref int) => write!(f, "{}", int),
            Type::Bool(ref b) => write!(f, "{}", b),
            Type::Str(ref s) => write!(f, "{}", s),
            Type::Seq(ref seq) => {
                if let Some(value) = seq.last() {
                    write!(f, "{:?}", value)
                } else {
                    write!(f, "()")
                }
            }
            Type::Fn(ref args, ref body, ref scope) => write!(f, "")
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Num {
    Int(isize),
    Float(f64),
}
