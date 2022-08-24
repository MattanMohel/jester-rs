use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone, Copy)]
pub struct Id {
    id: usize
}

impl Id {
    pub fn new() -> Self {
        Self { 
            id: Self::next_id()
        }
    }

    pub fn next_id() -> usize {
        static ID: AtomicUsize = AtomicUsize::new(0);
        ID.fetch_add(1, Ordering::Relaxed) 
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}