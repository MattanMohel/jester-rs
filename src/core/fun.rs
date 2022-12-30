use super::{
    id::Id, 
    env::Env, 
    err::Err,
    obj::Obj,
    node::{Node, NodeIter}, 
};

pub type Bridge = fn(&Env, NodeIter) -> Err<Obj>;

pub trait Callable {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj>;
    fn name(&self) -> &String;
}

#[derive(Clone)]
pub struct FnNative {
    name: String,
    params: Node,
    body: Node,
    id: Id
}

impl Callable for FnNative {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        self.body
            .iter()
            .progn_scoped(env, self.params.iter(), args)
    }

    fn name(&self) -> &String {
        &self.name
    }
}

impl PartialEq for FnNative {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FnNative {
    pub fn new(name: String, params: Node, body: Node) -> Self {
        Self {
            name,
            params,
            body,
            id: Id::new()
        }
    }

    pub fn params(&self) -> &Node {
        &self.params
    }
}

#[derive(Clone)]
pub struct FnBridge {
    name: String,
    body: Bridge
}

impl FnBridge {
    pub fn new(name: String, body: Bridge) -> Self {
        FnBridge { 
            name,
            body
        }
    }
}

impl Callable for FnBridge {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        (self.body)(env, args)
    }

    fn name(&self) -> &String {
        &self.name
    }
}