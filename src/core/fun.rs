use super::{
    id::Id, 
    env::Env, 
    obj::Obj,
    type_id::TypeId,
    rc_cell::RcCell,
    err::{Err, ErrType::*},
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
    fold: bool,
    id: Id
}

impl Callable for FnNative {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        let delta = args.len().checked_sub(self.params.len());

        match (delta, self.fold) {
            (None, _) => Err(Params),

            (Some(n), false) if n > 0 => Err(Params),

            (Some(n), true) => {
                let fold = args
                    .skip(args.len() - n - 1)
                    .map(|obj| env.eval(obj.as_ref()))
                    .collect::<Err<Node>>()?
                    .as_obj();

                let node = vec![fold.into()];

                let fold_args = args
                    .take(args.len() - n - 1)
                    .chain(node.iter());

                self
                    .body
                    .iter()
                    .progn_scoped(env, self.params.iter(), fold_args)
            }

            (Some(_), false) => {
                self.body
                    .iter()
                    .progn_scoped(env, self.params.iter(), args)
            }
        }
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
    pub fn new(name: String, params: Node, body: Node, fold: bool) -> Self {
        Self {
            name,
            params,
            body,
            fold,
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

#[derive(Clone)]
pub struct FnMacro {
    name: String,
    params: Node,
    body: Node,
    fold: bool,
    id: Id
}

impl Callable for FnMacro {
    fn call(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        // store macro expansion
        let exp = self.expand(env, args)?;
        env.eval(&exp)
    }

    fn name(&self) -> &String {
        &self.name
    }
}

impl PartialEq for FnMacro {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FnMacro {
    pub fn new(name: String, params: Node, body: Node, fold: bool) -> Self {
        Self {
            name,
            params,
            body,
            fold,
            id: Id::new()
        }
    }

    pub fn params(&self) -> &Node {
        &self.params
    }

    pub fn expand(&self, env: &Env, args: NodeIter) -> Err<Obj> {
        let delta = args.len().checked_sub(self.params.len());

        match (delta, self.fold) {
            (None, _) => Err(Params),

            (Some(n), false) if n > 0 => Err(Params),

            (Some(n), true) => {
                let fold = args
                    .skip(args.len() - n - 1)
                    .cloned()
                    .collect::<Node>()
                    .as_obj();

                let node = vec![fold.into()];

                let fold_args = args
                    .take(args.len() - n - 1)
                    .chain(node.iter());

                self
                    .body
                    .iter()
                    .progn_macro(env, self.params.iter(), fold_args)
            }

            (Some(_), false) => {
                self.body
                    .iter()
                    .progn_macro(env, self.params.iter(), args)
            }
        }
    }
}