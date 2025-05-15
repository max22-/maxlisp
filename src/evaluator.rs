pub mod env;
use std::collections::VecDeque;

use builtins::global_env;
use env::Env;
use crate::{context::{gc_heap::Handle, Context}, sexp::{BuiltinFn, Sexp, Symbol}};
pub mod builtins;

#[derive(Debug)]
pub enum EvalError {
    StackUnderflow,
    QueueUnderflow,
    CantPushOperator,
    TypeError(String),
    SymbolNotBound(String),
}

pub enum EvalItem {
    Operator(BuiltinFn),
    Operand(Handle),
}

impl EvalItem {
    pub fn to_string(&self, ctx: &Context) -> String {
        match self {
            Self::Operator(_) => String::from("<op>"),
            Self::Operand(h) => ctx.heap.get_ref(*h).to_string(ctx)
        }
    }
}

pub struct Evaluator{
    stack: Vec<EvalItem>,
    queue: VecDeque<EvalItem>,
    env: Handle,
}

impl Evaluator{
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            stack: vec![],
            queue: VecDeque::new(),
            env: global_env(ctx)
        }
    }

    fn push(&mut self, handle: Handle) {
        self.stack.push(EvalItem::Operand(handle));
    }

    fn pop(&mut self) -> Result<Handle, EvalError> {
        match self.stack.pop() {
            Some(EvalItem::Operand(h)) => Ok(h),
            Some(_) => unreachable!(),
            None => Err(EvalError::StackUnderflow)
        }
    }

    pub fn push_back(&mut self, item: EvalItem) {
        self.queue.push_back(item);
    }

    fn pop_front(&mut self) -> Result<EvalItem, EvalError> {
        self.queue.pop_front().ok_or(EvalError::QueueUnderflow)
    }

    fn push_front(&mut self, mut q: VecDeque<EvalItem>) {
        q.append(&mut self.queue);
        self.queue = q;
    }

    pub fn lookup(&self, sym: Symbol, ctx: &Context) -> Option<Handle> {
        let env = ctx.heap.get_ref(self.env);
        match env {
            Sexp::Env(env) => env.lookup(sym, ctx),
            _ => unreachable!()
        }
    }

    pub fn run(&mut self, ctx: &mut Context) -> Result<(), EvalError>{
        loop {
            let item = self.queue.pop_front();
            match item {
                Some(item) => {
                    match item {
                        EvalItem::Operator(op) => op(self, ctx)?,
                        EvalItem::Operand(h) => self.push(h)
                    }
                },
                None => break
            }
            println!("{}", self.to_string(ctx));
        }
        println!("{}", self.to_string(ctx));
        Ok(())
    }

    pub fn to_string(&self, ctx: &Context) -> String {
        let mut result = String::from("[");
        for (i, item) in self.stack.iter().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            result.push_str(&item.to_string(ctx));
        }
        result.push_str("] : [");
        for (i, item) in self.queue.iter().enumerate() {
            if i != 0 {
                result.push(' ');
            }
            result.push_str(&item.to_string(ctx));
        }
        result.push(']');
        result
    }
}

