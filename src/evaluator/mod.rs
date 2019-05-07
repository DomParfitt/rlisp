use crate::parser::grammar::Expr;
use crate::lexer::TokenKind;
use crate::evaluator::scope::Scope;
use crate::evaluator::types::Type;
use std::rc::Rc;
use std::cell::RefCell;

mod scope;
pub mod types;

pub struct Evaluator {
    scope: Rc<RefCell<Scope>>
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            scope: Rc::new(RefCell::new(Scope::global())),
        }
    }

    pub fn evaluate(&mut self, expr: Expr) -> Type {
        self.eval_expr(expr, Rc::clone(&self.scope))
    }

    fn eval_expr(&mut self, expr: Expr, context: Rc<RefCell<Scope>>) -> Type {
        match expr {
            Expr::List(list) => self.eval_list(list, context),
            Expr::BinaryOp(op, left, right) => self.eval_bin_op(op, *left, *right),
            Expr::Bind(id, value) => self.eval_bind(*id, *value, context),
            Expr::FnDef(args, body) => self.eval_fn_def(*args, *body, context),
            Expr::If(condition, result, alternate) => self.eval_if_expr(*condition, *result, *alternate),
            Expr::Base(expr) => self.eval_expr(*expr, context),
            Expr::Ident(id) => self.eval_identifier(&id, context),
            Expr::Value(val) => Type::Int(val),
        }
    }

    fn eval_list(&mut self, list: Vec<Expr>, context: Rc<RefCell<Scope>>) -> Type {
        let mut results = vec!();
        for expr in list {
            results.push(self.eval_expr(expr, context));
        }

        Type::Seq(results)
    }

    fn eval_bin_op(&mut self, op: TokenKind, left: Expr, right: Expr) -> Type {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match (left.clone(), right.clone()) {
            (Type::Int(lval), Type::Int(rval)) => self.eval_bin_op_int(op, lval, rval),
            (Type::Bool(lval), Type::Bool(rval)) => self.eval_bin_op_bool(op, lval, rval),
            _ => self.error(format!("Operator: '{:?}' cannot be applied to arguments of type {:?} and {:?}.", op, left, right)),
        }
    }

    fn eval_bin_op_int(&self, op: TokenKind, left: isize, right: isize) -> Type {
        match op {
            TokenKind::Plus => Type::Int(left + right),
            TokenKind::Minus => Type::Int(left - right),
            TokenKind::Star => Type::Int(left * right),
            TokenKind::Div => Type::Int(left / right),
            TokenKind::Eq => Type::Bool(left == right),
            TokenKind::GreaterThan => Type::Bool(left > right),
            TokenKind::GreaterThanEq => Type::Bool(left >= right),
            TokenKind::LessThan => Type::Bool(left < right),
            TokenKind::LessThanEq => Type::Bool(left <= right),
            _ => self.error(format!("Operator: '{:?}' cannot be applied to arguments of type Int and Int.", op)),
        }
    }

    fn eval_bin_op_bool(&self, op: TokenKind, left: bool, right: bool) -> Type {
        match op {
            TokenKind::Eq => Type::Bool(left == right),
            _ => self.error(format!("Operator: '{:?}' cannot be applied to arguments of type Bool and Bool.", op)),
        }
    }

    fn eval_bind(&mut self, ident: Expr, value: Expr, context: Rc<RefCell<Scope>>) -> Type {
        if let Expr::Ident(id) = ident {
            let value = self.evaluate(value.clone());
            return context.borrow_mut().bind(id, value);
        }

        self.error(format!("Cannot bind to '{:?}' as it is not a valid identifier.", value))
    }

    fn eval_fn_def(&mut self, args: Expr, body: Expr, context: Rc<RefCell<Scope>>) -> Type {
        if let Expr::List(list) = args {
            let mut ids = vec!();
            for expr in list {
                if let Expr::Ident(id) = expr {
                    ids.push(id);
                    continue;
                }
                return self.error_str("Expected identifier");
            }

            return Type::Fn(ids, body, context);
        }

        self.error_str("Expected argument list")
    }

    fn eval_if_expr(&mut self, condition: Expr, result: Expr, alternate: Expr) -> Type {
        if self.truthy(condition) {
            self.evaluate(result)
        } else {
            self.evaluate(alternate)
        }
    }

    fn eval_identifier(&mut self, ident: &str, context: Rc<RefCell<Scope>>) -> Type {
        if let Some(value) = context.borrow_mut().get(ident) {
            return value;
        }

        self.error(format!("No binding for identifier '{}' found in the current scope.", ident))
    }

    fn truthy(&mut self, expr: Expr) -> bool {
        let result = self.evaluate(expr);
        match result {
            Type::Bool(value) => value,
            _ => false,
        }
    }

    fn error(&self, message: String) -> Type {
        Type::Err(message)
    }

    fn error_str(&self, message: &str) -> Type {
        self.error(message.to_string())
    }
}