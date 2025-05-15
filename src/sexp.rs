use crate::context::gc_heap::Handle;
use crate::context::Context;
use crate::evaluator::Evaluator;


pub type Symbol = u64;

pub enum Sexp {
    Integer(i64),
    Symbol(Symbol),
    String(String),
    Pair(Handle, Handle),
    Nil,
    Builtin(fn(&mut Evaluator)),
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
            Self::Builtin(_) => String::from("<builtin>")
        }
    }
}
