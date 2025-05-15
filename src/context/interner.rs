use crate::sexp::Symbol;
use std::collections::HashMap;
pub struct Interner {
    strings: HashMap<String, Symbol>,
    counter: Symbol,
}

impl Interner {
    pub fn new() -> Self {
        return Self {
            strings: HashMap::new(),
            counter: 0,
        };
    }

    pub fn intern(self: &mut Self, s: &str) -> Symbol {
        let s = String::from(s);
        if self.strings.contains_key(&s) {
            self.strings[&s]
        } else {
            let sym = self.counter;
            self.counter += 1;
            self.strings.insert(s, sym);
            sym
        }
    }

    pub fn string_from_symbol(self: &Self, s: Symbol) -> Option<&String> {
        for (k, v) in self.strings.iter() {
            if *v == s {
                return Some(k);
            }
        }
        return None;
    }
}
