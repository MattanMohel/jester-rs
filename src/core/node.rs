use std::{
    fmt,
    cell::{RefMut, Ref}, ops::{Deref, DerefMut}
};

use super::{
    env::Env,
    obj::Obj, 
    rc_cell::RcCell, 
    type_id::TypeId,
    err::{Err, ErrType::*, AsResult}
};



impl Default for Node {
    fn default() -> Self {
        Node { 
            vec: Vec::new() 
        }
    }
}

impl From<Vec<RcCell<Obj>>> for Node {
    fn from(items: Vec<RcCell<Obj>>) -> Self {
        Self {
            vec: items 
        }
    }
}

impl FromIterator<RcCell<Obj>> for Node {
    fn from_iter<T>(iter: T) -> Self 
        where 
            T: IntoIterator<Item=RcCell<Obj>>
    {
        Node::from(
            iter
                .into_iter()
                .collect::<Vec<_>>())
    }
}

impl FromIterator<Obj> for Node {
    fn from_iter<T>(iter: T) -> Self 
        where 
            T: IntoIterator<Item=Obj>
    {
        Node::from(
            iter
                .into_iter()
                .map(|obj| RcCell::from(obj))
                .collect::<Vec<_>>())
    }
}
    
#[derive(Clone, PartialEq)]
pub struct Node {
    vec: Vec<RcCell<Obj>>,
}

impl TypeId for Node {
    fn into_obj(self) -> Obj {
        Obj::Lst(self)
    }
}

impl Node {
    pub fn to_string(&self, env: &Env) -> String {
        self.vec
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, obj)|
                if i == 0 {
                    format!("{}", obj.as_ref().to_string(env).as_str())
                } 
                else if i + 1 == self.len() {
                    format!("({} {})", acc, obj.as_ref().to_string(env).as_str())
                }
                else {
                    format!("{} {}", acc, obj.as_ref().to_string(env).as_str())
                })
    }

    pub fn get(&self, i: usize) -> Err<&RcCell<Obj>> {
        self.vec
            .get(i)
            .ok_or(OutOfBound)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    /// Creates an evaluated copy of Node
    pub fn evaled(&self, env: &Env) -> Err<Self> {
        self
            .iter()
            .map(|obj| env.eval(obj.as_ref().deref()))
            .collect::<Err<Node>>()
    }

    pub fn iter(&self) -> NodeIter<'_> {
        NodeIter::new(self, 0)
    }

    pub fn iter_from(&self, offset: usize) -> NodeIter<'_> {
        NodeIter::new(self, offset)
    }
}


#[derive(Copy)]
pub struct NodeIter<'a> {
    node: &'a Node,
    offset: usize,
    i: usize,
}

impl<'a> Clone for NodeIter<'a> {
    fn clone(&self) -> Self {
        Self { 
            node: self.node, 
            offset: self.offset.clone(), 
            i: 0 
        }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a RcCell<Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.get(self.i - 1).ok()
    }
}

impl<'a> NodeIter<'a> {
    fn new(node: &'a Node, offset: usize) -> Self {
        NodeIter { 
            node: node, 
            offset: offset, 
            i: 0 
        }
    }

    pub fn get(&self, index: usize) -> Err<&'a RcCell<Obj>> {
        self.node.get(self.offset + index)
    }

    pub fn get_ref(&self, index: usize) -> Err<Ref<Obj>> {
        self.node
            .get(self.offset + index)
            .map(|item| item.as_ref())
    }

    pub fn get_mut(&self, index: usize) -> Err<RefMut<Obj>> {
        self.node
            .get(self.offset + index)
            .map(|item| item.as_mut())
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.node.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn shift(&self, offset: usize) -> Self {
        Self::new(self.node, self.offset + offset)
    }

    pub fn progn<F>(self, mut map: F) -> Err<Obj>  
    where F: FnMut(&'a RcCell<Obj>, bool) -> Err<Obj> 
    {        
        let bounds = 
            self
                .len()
                .checked_sub(self.offset + 1).unwrap_or(0);  

        for i in 0..bounds {
            map(self.get(i)?, false)?;
        }

        map(self.get(bounds)?, true)
    }

    pub fn progn_scoped<'b, 'c, It1, It2>(self, env: &Env, params: It1, args: It2) -> Err<Obj>     
    where 
        It1: Iterator<Item = &'b RcCell<Obj>> + Clone, 
        It2: Iterator<Item = &'c RcCell<Obj>> + Clone
    {   
        let prev = params.clone();   
        
        for (param, arg) in params.clone().zip(args.clone()) {   
            param
                .as_mut()
                .deref_mut()
                .set(&env.eval(arg.as_ref().deref())?);
        }

        let ret = self.progn(|obj, _| env.eval(obj.as_ref().deref()));

        for (param, arg) in params.zip(prev) {   
            param
                .as_mut()
                .deref_mut()
                .set(&env.eval(arg.as_ref().deref())?);
        }

        ret
    }
}