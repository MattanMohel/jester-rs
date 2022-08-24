use super::{rc_cell::RcCell, object::Obj};


#[derive(Clone)]
pub struct Node {
    args: Vec<RcCell<Obj>>,
}

impl Default for Node {
    fn default() -> Self {
        Node { 
            args: Vec::new() 
        }
    }
}
