use crate::context::gc_heap::Handle;
use crate::sexp::Symbol;
use std::collections::HashMap;
use std::vec;

pub struct Env {
    bindings: Vec<HashMap<Symbol, Handle>>,
}

impl Env {
    pub fn new() -> Self {
        Self { bindings: vec![] }
    }

    pub fn push_empty(self: &mut Self) {
        self.bindings.push(HashMap::new());
    }

    pub fn pop(self: &mut Self) {
        self.bindings.pop().expect("environment stack underflow");
    }

    pub fn def(self: &mut Self, sym: Symbol, handle: Handle) {
        self.bindings
            .last_mut()
            .expect("empty environment stack")
            .insert(sym, handle);
    }

    pub fn lookup(self: &Self, sym: Symbol) -> Option<Handle> {
        for e in self.bindings.iter().rev() {
            match e.get(&sym) {
                Some(handle) => return Some(*handle),
                None => ()
            }
        }
        None
    }
}
