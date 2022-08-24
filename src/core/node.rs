use std::{cell::{RefMut, Ref, RefCell}, rc::Rc};

use super::{err::{Err, ErrType::*, AsResult}, rc_cell::RcCell, object::Obj, type_id::TypeId};


#[derive(Clone)]
pub struct Node {
    vec: Vec<RcCell<Obj>>,
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
    where T: IntoIterator<Item =  RcCell<Obj>>
    {
        Node::from(
            iter
                .into_iter()
                .collect::<Vec<RcCell<Obj>>>())
    }
}

impl Default for Node {
    fn default() -> Self {
        Node { 
            vec: Vec::new() 
        }
    }
}

impl TypeId for Node {
    fn into_obj(self) -> Obj {
        Obj::Lst(self)
    }
}

impl Node {
    pub fn get(&self, i: usize) -> Err<&RcCell<Obj>> {
        self.vec
            .get(i)
            .ok_or(OutOfBound)
    }

    pub fn len(&self) -> usize {
        self.vec.len()
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

    pub fn shift(&self, offset: usize) -> Self {
        Self::new(self.node, self.offset + offset)
    }

    pub fn do_list<F>(self, mut map: F) -> Err<Obj>  
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

    // pub fn scope<'b, 'c, It1, It2>(env: &Env, params: &'b It1, args: &'c It2, body: NodeIter) -> Err<Obj>     
    // where 
    // It1: Iterator<Item = &'b RcCell<Obj>> + Clone, 
    // It2: Iterator<Item = &'c RcCell<Obj>> + Clone
    // {   
    //     let params = 
    //         params
    //             .clone()
    //             .collect::<Vec<_>>();
        
    //     let args = 
    //         args
    //             .clone()
    //             .collect::<Vec<_>>();

    //     // (params.len() != args.len())
    //     //     .as_result((), UnmatchedParamLists)?;

    //     let prev = 
    //         params
    //             .iter()
    //             .map(|item| item.as_ref().clone())
    //             .collect::<Vec<Obj>>();
        
    //     for (param, arg) in params.iter().zip(args.iter()) {
    //         let eval = env.eval(arg.as_ref().deref())?
            
    //         param
    //             .as_mut()
    //             .set(&eval);
    //     }

    //     let ret = 
    //         body   
    //             .do_list(|obj, _| {
    //                 env.eval(obj.as_ref().deref())
    //             });

    //     for (param, arg) in params.iter().zip(prev.iter()) {
    //         param
    //             .as_mut()
    //             .set(arg);
    //     }

    //     ret
    // }
}