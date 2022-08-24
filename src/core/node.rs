use super::{err::{Err, ErrType::*, AsResult}, rc_cell::RcCell, object::Obj};


#[derive(Clone)]
pub struct Node {
    vec: Vec<RcCell<Obj>>,
}

impl Default for Node {
    fn default() -> Self {
        Node { 
            vec: Vec::new() 
        }
    }
}

impl Node {
    pub fn get(&self, i: usize) -> Err<&RcCell<Obj>> {
        self.vec
            .get(i)
            .ok_or(OutOfBound)
    }

    pub fn push(&mut self, item: RcCell<Obj>) {
        self.vec.push(item)
    }

    pub fn insert(&mut self, index: usize, item: RcCell<Obj>) {
        self.vec.insert(index, item)
    }

    pub fn remove(&mut self, index: usize) -> Err<Obj> {
        (index < self.vec.len()).ok_then(
            self.vec.remove(index).as_ref().clone(), 
            OutOfBound
        )
    }
}