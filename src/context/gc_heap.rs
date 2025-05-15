use crate::sexp::Sexp;
use std::vec;

const MAX_HEAP_SIZE: usize = 1000;

pub type Handle = usize;

pub struct Cell {
    val: Option<Sexp>,
    mark: bool,
}

impl Cell {
    fn new(sexp: Sexp) -> Self {
        return Self {
            val: Some(sexp),
            mark: false,
        };
    }
}

pub struct GcHeap {
    cells: Vec<Cell>,
    free_list: Vec<Handle>,
}

impl GcHeap {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            free_list: vec![],
        }
    }

    pub fn alloc(self: &mut Self, sexp: Sexp) -> Handle {
        match self.free_list.pop() {
            Some(handle) => {
                self.cells.get_mut(handle).expect("unknown id").val = Some(sexp);
                handle
            }
            None => {
                let handle = self.cells.len();
                self.cells.push(Cell::new(sexp));
                handle
            }
        }
    }

    pub fn get_ref(self: &Self, handle: Handle) -> &Sexp {
        self.cells
            .get(handle)
            .expect("unknown id")
            .val
            .as_ref()
            .expect("empty cell")
    }

    pub fn get_mut_ref(self: &mut Self, handle: Handle) -> &mut Sexp {
        self.cells
            .get_mut(handle)
            .expect("unknown id")
            .val
            .as_mut()
            .expect("empty cell")
    }
}

pub trait Mark {
    fn mark(&mut self);
}
