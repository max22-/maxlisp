pub mod gc_heap;
mod interner;
use gc_heap::GcHeap;
use interner::Interner;

pub struct Context {
    pub heap: GcHeap,
    pub interner: Interner
}

impl Context {
    pub fn new() -> Self {
        Self {
            heap: GcHeap::new(),
            interner: Interner::new()
        }
    }
}