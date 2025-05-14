use core::fmt;
use crate::interner::Interner;
use crate::gc_heap::{GcHeap, Handle};
pub enum Sexp {
    Integer(i64),
    Symbol(u64),
    String(String),
    Pair(Handle, Handle),
    Nil,
}

impl Sexp {
    pub fn to_string(&self, heap: &GcHeap, interner: &Interner) -> String {
        match self {
            Sexp::Integer(i) => format!("{}", i),
            Sexp::Symbol(s) => format!("{}", interner.string_from_symbol(*s).unwrap_or(&String::from("<unknown symbol>"))),
            Sexp::String(s) => format!("{:?}", s),
            Sexp::Pair(car_handle, cdr_handle) => {
                let car = heap.get_ref(*car_handle);
                let cdr = heap.get_ref(*cdr_handle);
                let mut result = String::new();
                result.push_str("(");
                result.push_str(car.to_string(heap, interner).as_str());
                let mut it = cdr;
                loop {
                    match it {
                        Sexp::Pair(car_handle, cdr_handle) => {
                            let car = heap.get_ref(*car_handle);
                            let cdr = heap.get_ref(*cdr_handle);
                            result.push(' ');
                            result.push_str(car.to_string(heap, interner).as_str());
                            it = &cdr;
                        }
                        Sexp::Nil => break,
                        s => {
                            result.push_str(" . ");
                            result.push_str(s.to_string(heap, interner).as_str());
                            break;
                        }
                    }
                }
                result.push(')');
                result
            }
            Sexp::Nil => String::from("()"),
        }
    }
}
