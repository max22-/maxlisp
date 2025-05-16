use crate::context::Context;
use crate::context::gc_heap::Handle;
use crate::evaluator::env::Env;
use crate::evaluator::{EvalError, Evaluator};

pub type Symbol = u64;
pub type BuiltinFn = fn(&mut Evaluator, &mut Context) -> Result<(), EvalError>;

pub struct Closure {
    pub env: Handle,
    pub vars: Vec<Handle>,
    pub sym: Handle,
    pub body: Handle,
}

pub enum Sexp {
    Integer(i64),
    Symbol(Symbol),
    String(String),
    Pair(Handle, Handle),
    Nil,
    Builtin(BuiltinFn),
    Env(Env),
    Closure(Closure),
}

impl Sexp {
    pub fn to_string(&self, ctx: &Context) -> String {
        match self {
            Sexp::Integer(i) => format!("{}", i),
            Sexp::Symbol(s) => format!(
                "{}",
                ctx.interner
                    .string_from_symbol(*s)
                    .unwrap_or(&String::from("<unknown symbol>"))
            ),
            Sexp::String(s) => format!("{:?}", s),
            Sexp::Pair(car_handle, cdr_handle) => {
                let car = ctx.heap.get_ref(*car_handle);
                let cdr = ctx.heap.get_ref(*cdr_handle);
                let mut result = String::new();
                result.push_str("(");
                result.push_str(car.to_string(ctx).as_str());
                let mut it = cdr;
                loop {
                    match it {
                        Sexp::Pair(car_handle, cdr_handle) => {
                            let car = ctx.heap.get_ref(*car_handle);
                            let cdr = ctx.heap.get_ref(*cdr_handle);
                            result.push(' ');
                            result.push_str(car.to_string(ctx).as_str());
                            it = &cdr;
                        }
                        Sexp::Nil => break,
                        s => {
                            result.push_str(" . ");
                            result.push_str(s.to_string(ctx).as_str());
                            break;
                        }
                    }
                }
                result.push(')');
                result
            }
            Sexp::Nil => String::from("()"),
            Sexp::Builtin(_) => String::from("<builtin>"),
            Sexp::Env(_) => String::from("<env>"),
            Sexp::Closure(_) => String::from("<closure>"),
        }
    }

    pub fn into_list<'a>(self: &Self, ctx: &'a Context) -> Result<Vec<&'a Sexp>, EvalError> {
        let mut list: Vec<&Sexp> = vec![];
        match self {
            Sexp::Pair(car_h, cdr_h) => {
                list.push(ctx.heap.get_ref(*car_h));
                let cdr = ctx.heap.get_ref(*cdr_h);
                let mut rest = cdr.into_list(ctx)?;
                list.append(&mut rest);
                Ok(list)
            }
            Sexp::Nil => Ok(list),
            _ => Err(EvalError::TypeError(String::from("expected a list"))),
        }
    }

    pub fn into_handle_list(self: &Self, ctx: &Context) -> Result<Vec<Handle>, EvalError> {
        let mut list: Vec<Handle> = vec![];
        match self {
            Sexp::Pair(car_h, cdr_h) => {
                list.push(*car_h);
                let cdr = ctx.heap.get_ref(*cdr_h);
                let mut rest = cdr.into_handle_list(ctx)?;
                list.append(&mut rest);
                Ok(list)
            }
            Sexp::Nil => Ok(list),
            _ => Err(EvalError::TypeError(String::from("expected a list"))),
        }
    }

    pub fn into_integer(self: &Self, ctx: &Context) -> Result<i64, EvalError> {
        match self {
            Sexp::Integer(i) => Ok(*i),
            _ => Err(EvalError::TypeError(String::from("expected an integer"))),
        }
    }

    pub fn from_handle_list(l: Vec<Handle>, ctx: &mut Context) -> Handle {
        let mut result = ctx.heap.alloc(Sexp::Nil);
        for i in l.iter().rev() {
            result = ctx.heap.alloc(Sexp::Pair(*i, result));
        }
        result
    }
}

