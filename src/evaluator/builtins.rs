use crate::evaluator::Evaluator;
use crate::evaluator::env::Env;
use crate::gc_heap::GcHeap;
use crate::interner::Interner;
use crate::sexp::Sexp;

pub fn add(e: &mut Evaluator) {

}

pub fn global_env(heap: &mut GcHeap, interner: &mut Interner) -> Env {
    let mut env = Env::new();
    env.push_empty();
    env.def(interner.intern(String::from("+")), heap.alloc(Sexp::Builtin(add)));
    env
}
