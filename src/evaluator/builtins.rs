use std::collections::VecDeque;

use crate::context::{Context, gc_heap::Handle};
use crate::evaluator::Evaluator;
use crate::evaluator::env::Env;
use crate::sexp::{Closure, Sexp};

use super::{EvalError, EvalItem};

pub fn add(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_list(ctx)?;
    let mut result: i64 = 0;
    for a in args {
        result += a
            .into_integer(ctx)
            .map_err(|_| EvalError::TypeError(String::from("expected a list of integers")))?;
    }
    e.push(ctx.heap.alloc(Sexp::Integer(result)));
    Ok(())
}

pub fn vau(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_handle_list(ctx)?;
    if args.len() != 3 {
        return Err(EvalError::InvalidNumberOfArguments);
    }

    let vars = ctx.heap.get_ref(args[0]).into_handle_list(ctx)?;

    let sym = args[1];
    match ctx.heap.get_ref(sym) {
        Sexp::Symbol(_) => (),
        _ => {
            return Err(EvalError::TypeError(String::from(
                "expected a symbol for the environment",
            )));
        }
    };

    let body = args[2];

    let closure = Closure {
        env: e.get_env(),
        vars: vars,
        sym: sym,
        body: body,
    };
    e.push(ctx.heap.alloc(Sexp::Closure(closure)));
    Ok(())
}

pub fn def_raw(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_handle_list(ctx)?;
    if args.len() != 2 {
        return Err(EvalError::InvalidNumberOfArguments);
    }
    let sym = match ctx.heap.get_ref(args[0]) {
        Sexp::Symbol(sym) => *sym,
        _ => return Err(EvalError::TypeError(String::from("expected symbol"))),
    };
    let val = args[1];
    e.define(sym, val, ctx);
    Ok(())
}

pub fn def(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_handle_list(ctx)?;
    if args.len() != 2 {
        return Err(EvalError::InvalidNumberOfArguments);
    }
    let sym_h = args[0];
    let val = args[1];
    let q = VecDeque::from(vec![
        EvalItem::Operand(sym_h),
        EvalItem::Operand(val),
        EvalItem::Operator(eval, "eval"),
        EvalItem::Operand(e.get_nil()),
        EvalItem::Operator(cons, "cons"),
        EvalItem::Operator(cons, "cons"),
        EvalItem::Operator(def_raw, "def_raw"),
    ]);
    e.push_front(q);
    Ok(())
}

pub fn cons(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let cdr = e.pop()?;
    let car = e.pop()?;
    e.push(ctx.heap.alloc(Sexp::Pair(car, cdr)));
    Ok(())
}

pub fn map_eval(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let l = ctx.heap.get_ref(e.pop()?).into_handle_list(ctx)?;
    let mut q = VecDeque::new();
    for i in &l {
        q.push_back(EvalItem::Operand(*i));
        q.push_back(EvalItem::Operator(eval, "eval"));
    }
    q.push_back(EvalItem::Operand(e.get_nil()));
    for _ in 0..l.len() {
        q.push_back(EvalItem::Operator(cons, "cons"));
    }
    Ok(())
}

pub fn wrap(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args = ctx.heap.get_ref(e.pop()?).into_handle_list(ctx)?;
    if args.len() != 1 {
        return Err(EvalError::InvalidNumberOfArguments);
    }
    let p = args[0];
    let closure = Closure {
        env: e.get_env(),
        vars: vec![ctx.heap.alloc(Sexp::Symbol(ctx.interner.intern("l")))],
        sym: ctx.heap.alloc(Sexp::Symbol(ctx.interner.intern("%"))),
        body: Sexp::from_handle_list(vec![ctx.heap.alloc(Sexp::Builtin(map_eval))], ctx),
    };
    let q = VecDeque::from(vec![
        EvalItem::Operand(p),
        EvalItem::Operator(eval, "eval"),
        EvalItem::Operand(ctx.heap.alloc(Sexp::Closure(closure))),
        EvalItem::Operator(apply, "apply"),
        EvalItem::Operator(apply, "apply"),
    ]);
    e.push_front(q);
    Ok(())
}

pub fn eval(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let h = e.pop()?;
    let sexp = ctx.heap.get_ref(h);
    match sexp {
        Sexp::Integer(_) => e.push(h),
        Sexp::Symbol(sym) => match e.lookup(*sym, ctx) {
            Some(h) => e.push(h),
            None => {
                return Err(EvalError::SymbolNotBound(
                    ctx.interner
                        .string_from_symbol(*sym)
                        .unwrap_or(&String::from("??"))
                        .clone(),
                ));
            }
        },
        Sexp::String(_) => e.push(h),
        Sexp::Pair(car, cdr) => {
            let mut q: VecDeque<EvalItem> = VecDeque::new();
            q.push_back(EvalItem::Operand(*car));
            q.push_back(EvalItem::Operator(eval, "eval"));
            q.push_back(EvalItem::Operator(push, "push"));
            q.push_back(EvalItem::Operand(*cdr));
            q.push_back(EvalItem::Operator(apply, "apply"));
            e.push_front(q);
        }
        Sexp::Nil => todo!(),
        Sexp::Builtin(f) => f(e, ctx)?,
        Sexp::Env(_) => todo!(),
        Sexp::Closure(_) => todo!(),
    }
    Ok(())
}

pub fn push(e: &mut Evaluator, _: &mut Context) -> Result<(), EvalError> {
    match e.pop_front()? {
        EvalItem::Operand(h) => {
            e.push(h);
            Ok(())
        }
        EvalItem::Operator(_, _) => Err(EvalError::CantPushOperator),
    }
}

pub fn push_env(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let args = ctx.heap.get_ref(args_h).into_handle_list(ctx)?;
    if args.len() != 1 {
        return Err(EvalError::InvalidNumberOfArguments);
    }
    e.push_env(args[0]);
    Ok(())
}

pub fn pop_env(e: &mut Evaluator, _: &mut Context) -> Result<(), EvalError> {
    e.pop_env()?;
    Ok(())
}

pub fn apply(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args_h = e.pop()?;
    let proc_h = e.pop()?;
    let proc = ctx.heap.get_ref(proc_h);
    let mut q = VecDeque::new();
    match proc {
        Sexp::Builtin(func) => {
            q.push_back(EvalItem::Operand(args_h));
            q.push_back(EvalItem::Operator(*func, "builtin func"));
        }
        Sexp::Closure(c) => {
            let args = ctx.heap.get_ref(args_h).into_handle_list(ctx)?;
            if c.vars.len() != args.len() {
                return Err(EvalError::InvalidNumberOfArguments);
            }
            q.push_back(EvalItem::Operand(c.env));
            q.push_back(EvalItem::Operand(e.get_nil()));
            q.push_back(EvalItem::Operator(cons, "cons"));
            q.push_back(EvalItem::Operator(push_env, "push_env"));
            for (var, val) in c.vars.iter().zip(args) {
                q.push_back(EvalItem::Operand(*var));
                q.push_back(EvalItem::Operand(val));
                q.push_back(EvalItem::Operand(e.get_nil()));
                q.push_back(EvalItem::Operator(cons, "cons"));
                q.push_back(EvalItem::Operator(cons, "cons"));
                q.push_back(EvalItem::Operator(def_raw, "def_raw"));
            }
            q.push_back(EvalItem::Operand(c.sym));
            q.push_back(EvalItem::Operand(e.get_env()));
            q.push_back(EvalItem::Operand(e.get_nil()));
            q.push_back(EvalItem::Operator(cons, "cons"));
            q.push_back(EvalItem::Operator(cons, "cons"));
            q.push_back(EvalItem::Operator(def_raw, "def_raw"));
            q.push_back(EvalItem::Operand(c.body));
            q.push_back(EvalItem::Operator(eval, "eval"));
            q.push_back(EvalItem::Operator(pop_env, "pop_env"));
        }
        _ => return Err(EvalError::TypeError(String::from("expected a procedure"))),
    };
    e.push_front(q);
    Ok(())
}

pub fn car(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args = e.pop()?;
    match ctx.heap.get_ref(args) {
        Sexp::Pair(car, _) => {
            e.push(*car);
            Ok(())
        }
        Sexp::Nil => Err(EvalError::TypeError(String::from(
            "Can't apply car to the empty list",
        ))),
        _ => Err(EvalError::TypeError(String::from("expected a list"))),
    }
}

pub fn cdr(e: &mut Evaluator, ctx: &mut Context) -> Result<(), EvalError> {
    let args = e.pop()?;
    match ctx.heap.get_ref(args) {
        Sexp::Pair(_, cdr) => {
            e.push(*cdr);
            Ok(())
        }
        Sexp::Nil => Err(EvalError::TypeError(String::from(
            "Can't apply cdr to the empty list",
        ))),
        _ => Err(EvalError::TypeError(String::from("expected a list"))),
    }
}

pub fn global_env(ctx: &mut Context) -> Handle {
    let mut env = Env::new(None);
    env.def(
        ctx.interner.intern("add"),
        ctx.heap.alloc(Sexp::Builtin(add)),
    );
    env.def(
        ctx.interner.intern("eval"),
        ctx.heap.alloc(Sexp::Builtin(eval)),
    );
    env.def(
        ctx.interner.intern("vau"),
        ctx.heap.alloc(Sexp::Builtin(vau)),
    );
    env.def(
        ctx.interner.intern("def"),
        ctx.heap.alloc(Sexp::Builtin(def)),
    );
    env.def(
        ctx.interner.intern("wrap"),
        ctx.heap.alloc(Sexp::Builtin(wrap)),
    );
    env.def(
        ctx.interner.intern("car"),
        ctx.heap.alloc(Sexp::Builtin(car)),
    );
    env.def(
        ctx.interner.intern("cdr"),
        ctx.heap.alloc(Sexp::Builtin(cdr)),
    );
    ctx.heap.alloc(Sexp::Env(env))
}
