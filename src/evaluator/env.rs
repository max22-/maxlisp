use crate::context::gc_heap::Handle;
use crate::context::Context;
use crate::sexp::{Sexp, Symbol};
use std::collections::HashMap;

pub struct Env {
    bindings: HashMap<Symbol, Handle>,
    outer: Option<Handle>
}

impl Env {
    pub fn new(outer: Option<Handle>) -> Self {
        Self { bindings: HashMap::new(), outer: outer }
    }

    pub fn def(self: &mut Self, sym: Symbol, handle: Handle) {
        self.bindings.insert(sym, handle);
    }

    pub fn lookup(self: &Self, sym: Symbol, ctx: &Context) -> Option<Handle> {
        match self.bindings.get(&sym) {
            Some(handle) => Some(*handle),
            None => match self.outer {
                Some(env_h) => {
                    if let Sexp::Env(env) = ctx.heap.get_ref(env_h) {
                        env.lookup(sym, ctx)
                    } else {
                        unreachable!()
                    }
                },
                None => None
            }
        }
    }
}
