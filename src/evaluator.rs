mod env;
use std::collections::VecDeque;

use env::Env;
use crate::{gc_heap::{GcHeap, Handle}, interner::{self, Interner}, sexp::Sexp};
mod builtins;

pub enum EvalError {
    StackUnderflow,
    QueueUnderflow
}

pub struct Evaluator{
    env: Env,
    stack: Vec<Handle>,
    queue: VecDeque<Handle>,
}

impl Evaluator{
    pub fn new(heap: &mut GcHeap, interner: &mut Interner) -> Self {
        Self {
            env: builtins::global_env(heap,  interner),
            stack: vec![],
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, handle: Handle) {
        self.stack.push(handle);
    }

    fn pop(&mut self) -> Result<Handle, EvalError> {
        self.stack.pop().ok_or(EvalError::StackUnderflow)
    }

    pub fn push_back(&mut self, handle: Handle) {
        self.queue.push_back(handle);
    }

    fn pop_front(&mut self) -> Result<Handle, EvalError> {
        self.queue.pop_front().ok_or(EvalError::QueueUnderflow)
    }

    fn push_front(&mut self, handle: Handle) {
        self.queue.push_front(handle);
    }

    pub fn eval(&mut self, heap: &mut GcHeap, interner: &mut Interner) {
        let eval_ = heap.alloc(Sexp::Symbol(interner.intern(String::from("eval"))));
        let push_ = heap.alloc(Sexp::Symbol(interner.intern(String::from("push"))));
        let apply_ = heap.alloc(Sexp::Symbol(interner.intern(String::from("apply"))));
        loop {
            let h = self.queue.pop_front();
            match h {
                Some(h) => {
                    let sexp = heap.get_ref(h);
                    match sexp {
                        Sexp::Integer(_) => self.stack.push(h),
                        Sexp::Symbol(_) => self.stack.push(h),
                        Sexp::String(_) => self.stack.push(h),
                        Sexp::Pair(car, cdr) => {
                            let mut q: VecDeque<Handle> = VecDeque::new();
                            q.push_back(*car);
                            q.push_back(eval_);
                            q.push_back(push_);
                            q.push_back(*cdr);
                            q.push_back(apply_);
                            q.append(&mut self.queue);
                            self.queue = q;
                        },
                        Sexp::Nil => todo!(),
                        Sexp::Builtin(_) => self.stack.push(h),
                    }
                },
                None => break
            }
            println!("{}", self.to_string(heap, interner));
        }
        println!("{}", self.to_string(heap, interner));
    }

    pub fn to_string(&self, heap: &mut GcHeap, interner: &mut Interner) -> String {
        let mut result = String::from("[");
        for (i, h) in self.stack.iter().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            result.push_str(&heap.get_ref(*h).to_string(heap, interner));
        }
        result.push_str("] : [");
        for (i, h) in self.queue.iter().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            result.push_str(&heap.get_ref(*h).to_string(heap, interner));
        }
        result.push(']');
        result
    }
}

