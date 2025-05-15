use crate::context::Context;
use crate::evaluator::Evaluator;
use crate::evaluator::env::Env;
use crate::sexp::Sexp;

pub fn add(e: &mut Evaluator) {

}

pub fn global_env(ctx: &mut Context) -> Env {
    let mut env = Env::new();
    env.push_empty();
    env.def(ctx.interner.intern("+"), ctx.heap.alloc(Sexp::Builtin(add)));
    env
}
