use core::fmt;

pub enum Sexp {
    Integer(i64),
    Symbol(u64),
    String(String),
    Pair(Box<Sexp>, Box<Sexp>),
    Nil,
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sexp::Integer(i) => write!(f, "{}", i),
            Sexp::Symbol(s) => write!(f, "{}", s),
            Sexp::String(s) => write!(f, "{:?}", s),
            Sexp::Pair(car, cdr) => {
                write!(f, "(")?;
                write!(f, "{}", car)?;
                let mut it = cdr.as_ref();
                loop {
                    match it {
                        Sexp::Pair(car, cdr) => {
                            write!(f, " {}", car)?;
                            it = &cdr;
                        }
                        Sexp::Nil => break,
                        s => {
                            write!(f, " . {}", s)?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            Sexp::Nil => write!(f, "()"),
        }
    }
}
