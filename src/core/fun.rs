use super::{
    id::Id, 
    env::Env, 
    err::Err,
    obj::Obj,
    type_id::TypeId,
    node::{Node, NodeIter}, 
};

pub trait Callable {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj>;
}

impl Callable for FnNative {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        self.exec
            .iter()
            .progn_scoped(env, self.params.iter(), args)
    }
}

impl PartialEq for FnNative {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone)]
pub struct FnNative {
    exec: Node,
    params: Node,
    id: Id
}

pub type Bridge = fn(&Env, NodeIter) -> Err<Obj>;

impl Callable for FnBridge {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        (self.exec)(env, args)
    }
}

impl PartialEq for FnBridge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


impl From<Bridge> for FnBridge {
    fn from(exec: Bridge) -> Self {
        FnBridge { 
            exec,
            id: Id::new()
        }
    }
}

#[derive(Clone)]
pub struct FnBridge {
    exec: Bridge,
    id: Id
}
