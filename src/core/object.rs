use super::{rc_cell::RcCell, node::Node};

#[derive(Clone)]
pub enum Obj {
    Sym(RcCell<Obj>),
    Lst(Node),
    
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(String),
    Nil(),
}