use core::fmt;
use crate::interner::Interner;
pub enum Sexp {
    Integer(i64),
    Symbol(u64),
    String(String),
    Pair(Box<Sexp>, Box<Sexp>),
    Nil,
}

impl Sexp {
    pub fn to_string(&self, interner: &Interner) -> String {
        match self {
            Sexp::Integer(i) => format!("{}", i),
            Sexp::Symbol(s) => format!("{}", interner.string_from_symbol(*s).unwrap_or(&String::from("<unknown symbol>"))),
            Sexp::String(s) => format!("{:?}", s),
            Sexp::Pair(car, cdr) => {
                let mut result = String::new();
                result.push_str("(");
                result.push_str(&car.to_string(interner));
                let mut it = cdr.as_ref();
                loop {
                    match it {
                        Sexp::Pair(car, cdr) => {
                            result.push(' ');
                            result.push_str(car.to_string(interner).as_str());
                            it = &cdr;
                        }
                        Sexp::Nil => break,
                        s => {
                            result.push_str(" . ");
                            result.push_str(&s.to_string(interner));
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
