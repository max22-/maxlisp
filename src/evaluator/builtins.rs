use std::collections::VecDeque;

use crate::context::Context;
use crate::evaluator::Evaluator;
use crate::evaluator::env::Env;
use crate::sexp::Sexp;

use super::{EvalError, EvalItem};

pub fn add(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    todo!()
}

pub fn eval(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let env_h = match e.pop()? {
        EvalItem::Operator(_) => unreachable!(),
        EvalItem::Operand(h) => h
    };
    let env = if let Sexp::Env(env) = ctx.heap.get_ref(env_h) {
        env
    } else {
        return Err(EvalError::TypeError(String::from("expected an environment")));
    };
    match e.pop()? {
        EvalItem::Operator(_) => unreachable!(),
        EvalItem::Operand(h) => {
            let sexp = ctx.heap.get_ref(h);
            match sexp {
                Sexp::Integer(_) => e.push(h),
                Sexp::Symbol(sym) => {
                    match env.lookup(*sym) {
                        Some(h) => e.push(h),
                        None => return Err(EvalError::SymbolNotBound(String::from("todo!")))
                    }
                },
                Sexp::String(_) => e.push(h),
                Sexp::Pair(car, cdr) => {
                    let mut q: VecDeque<EvalItem> = VecDeque::new();
                    q.push_back(EvalItem::Operand(*car));
                    q.push_back(EvalItem::Operand(env_h));
                    q.push_back(EvalItem::Operator(eval));
                    q.push_back(EvalItem::Operator(push));
                    q.push_back(EvalItem::Operand(*cdr));
                    q.push_back(EvalItem::Operator(apply));
                    q.append(&mut e.queue);
                    e.queue = q;
                },
                Sexp::Nil => todo!(),
                Sexp::Builtin(f) => f(e, ctx)?,
                Sexp::Env(_) => todo!()
            }
        }
    }
    e.push(env_h);
    Ok(())
}

pub fn push(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    match e.pop_front()? {
        EvalItem::Operand(h) => {
            e.push(h);
            Ok(())
        },
        EvalItem::Operator(_) => Err(EvalError::CantPushOperator)
    }
}

pub fn apply(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    Ok(())
}

pub fn global_env(ctx: &mut Context) -> Env {
    let mut env = Env::new();
    env.push_empty();
    env.def(ctx.interner.intern("+"), ctx.heap.alloc(Sexp::Builtin(add)));
    env.def(ctx.interner.intern("eval"), ctx.heap.alloc(Sexp::Builtin(eval)));
    env
}
