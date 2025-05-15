use std::{collections::VecDeque, env::Args};

use crate::context::{gc_heap::Handle, Context};
use crate::evaluator::Evaluator;
use crate::evaluator::env::Env;
use crate::sexp::Sexp;

use super::{EvalError, EvalItem};

pub fn add(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_list(ctx)?;
    let mut result: i64 = 0;
    for a in args {
        result += a.into_integer(ctx).map_err(|_| EvalError::TypeError(String::from("expected a list of integers")))?;
    }
    e.push(ctx.heap.alloc(Sexp::Integer(result)));
    Ok(())
}

pub fn wrap(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    Ok(())
}

pub fn eval(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let h = e.pop()?;
    let sexp = ctx.heap.get_ref(h);
    match sexp {
        Sexp::Integer(_) => e.push(h),
        Sexp::Symbol(sym) => {
            match e.lookup(*sym, ctx) {
                Some(h) => e.push(h),
                None => return Err(EvalError::SymbolNotBound(String::from("todo!")))
            }
        },
        Sexp::String(_) => e.push(h),
        Sexp::Pair(car, cdr) => {
            let mut q: VecDeque<EvalItem> = VecDeque::new();
            q.push_back(EvalItem::Operand(*car));
            q.push_back(EvalItem::Operator(eval));
            q.push_back(EvalItem::Operator(push));
            q.push_back(EvalItem::Operand(*cdr));
            q.push_back(EvalItem::Operator(apply));
            e.push_front(q);
        },
        Sexp::Nil => todo!(),
        Sexp::Builtin(f) => f(e, ctx)?,
        Sexp::Env(_) => todo!(),
        Sexp::Closure(_) => todo!()
    }
    Ok(())
}

pub fn push(e: &mut Evaluator, _: &mut Context) -> Result<(), EvalError> {
    match e.pop_front()? {
        EvalItem::Operand(h) => {
            e.push(h);
            Ok(())
        },
        EvalItem::Operator(_) => Err(EvalError::CantPushOperator)
    }
}

pub fn apply(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args = e.pop()?;
    let func_h = e.pop()?;
    let func = match ctx.heap.get_ref(func_h) {
        Sexp::Builtin(func) => func,
        _ => return Err(EvalError::TypeError(String::from("expected a procedure")))
    };
    e.push(args);
    func(e, ctx)?;
    Ok(())
}

pub fn global_env(ctx: &mut Context) -> Handle {
    let mut env = Env::new(None);
    env.def(ctx.interner.intern("+"), ctx.heap.alloc(Sexp::Builtin(add)));
    env.def(ctx.interner.intern("eval"), ctx.heap.alloc(Sexp::Builtin(eval)));
    ctx.heap.alloc(Sexp::Env(env))
}
