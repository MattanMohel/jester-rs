use std::ops::DerefMut;
use std::ops::Index;

use super::env::Env;
use super::obj::Obj;
use super::err::Err;
use super::err::ErrType::*;
use super::rc_cell::RcCell;
    
#[derive(Clone)]
pub struct Node {
    buf: Vec<RcCell<Obj>>,
}

impl Default for Node {
    fn default() -> Self {
        Node { 
            buf: Vec::new() 
        }
    }
}

impl From<Vec<RcCell<Obj>>> for Node {
    fn from(items: Vec<RcCell<Obj>>) -> Self {
        Self {
            buf: items 
        }
    }
}

impl FromIterator<RcCell<Obj>> for Node {
    fn from_iter<T>(iter: T) -> Self 
    where 
        T: IntoIterator<Item=RcCell<Obj>>
    {
        Node::from(iter
            .into_iter()
            .collect::<Vec<_>>())
    }
}

impl FromIterator<Obj> for Node {
    fn from_iter<T>(iter: T) -> Self 
    where 
        T: IntoIterator<Item=Obj>
    {
        Node::from(iter
            .into_iter()
            .map(|obj| RcCell::from(obj))
            .collect::<Vec<_>>())
    }
}

impl Node {

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: RcCell<Obj>) {
        self.buf.push(item)
    }

    pub fn insert(&mut self, i: usize, item: RcCell<Obj>) -> Err {
        if i >= self.len() {
            return Err(OutOfBound)
        }
        
        self.buf.insert(i, item);
        Ok(())
    }

    pub fn remove(&mut self, i: usize) -> Err<Obj> {
        if i >= self.len() {
            return Err(OutOfBound)
        }
        
        Ok(self.buf.remove(i).as_ref().clone())
    }

    pub fn get_cell(&self, i: usize) -> Err<&RcCell<Obj>> {
        self.buf
            .get(i)
            .ok_or(OutOfBound)
    }

    pub fn get_mut(&self, i: usize) -> Err<&mut Obj> {
        self.buf
            .get(i)
            .map(|obj| obj.as_mut())
            .ok_or(OutOfBound)
    }

    pub fn get(&self, i: usize) -> Err<&Obj> {
        self.buf
            .get(i)
            .map(|obj| obj.as_ref())
            .ok_or(OutOfBound)
    }

    /// Returns evaluated clone
    pub fn evaled(&self, env: &Env) -> Err<Node> {
        self.iter().mapped(|obj| env.eval(obj.as_ref()))
    }

    /// Creates an iterator
    pub fn iter(&self) -> NodeIter<'_> {
        NodeIter::new(self, 0)
    }

    /// Creates an iterator with offset `beg`
    pub fn iter_from(&self, beg: usize) -> NodeIter<'_> {
        NodeIter::new(self, beg)
    }
}

#[derive(Copy, Clone)]
pub struct NodeIter<'a> {
    node: &'a Node,
    beg: usize,
    i: usize,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a RcCell<Obj>;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.get_cell(self.i - 1).ok()
    }
}

impl<'a> Index<usize> for NodeIter<'a> {
    type Output = RcCell<Obj>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_cell(index).unwrap()
    }
}

impl<'a> NodeIter<'a> {
    fn new(node: &'a Node, beg: usize) -> Self {
        NodeIter { 
            node, 
            beg, 
            i: 0 
        }
    }
    
    pub fn len(&self) -> usize {
        self.node.len() - self.beg
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_cell(&self, i: usize) -> Err<&'a RcCell<Obj>> {
        self.node.get_cell(self.beg + i)
    }

    pub fn get_mut(&self, i: usize) -> Err<&mut Obj> {
        self.node.get_mut(self.beg + 1)
    }

    pub fn get(&self, i: usize) -> Err<&Obj> {
        self.node.get(self.beg + i)
    }

    pub fn shift(&self) -> Self {
        Self::new(self.node, self.beg + 1)
    }

    /// Return a new `Node` with elements mapped by `map`
    pub fn mapped<F>(&self, mut map: F) -> Err<Node> 
    where 
        F: FnMut(&'a RcCell<Obj>) -> Err<Obj> 
    {       
        self
            .map(|obj| map(obj))
            .collect::<_>()
    }

    /// Apply `map` to each element returning 
    /// the evaluation of the last element
    /// 
    /// ## Example
    /// ```
    /// // equivalent to "do"
    /// (set x (do 1, 2, 3, 4, 5))
    /// (assert-eq x 5)
    /// ```
    pub fn progn<F>(&self, mut map: F) -> Err<Obj> 
    where 
        F: FnMut(&'a RcCell<Obj>) -> Err<Obj> 
    {        
        let bounds = self
            .len()
            .checked_sub(1)
            .unwrap_or(0);  

        for i in 0..bounds {
            map(self.get_cell(i)?)?;
        }

        map(self.get_cell(bounds)?)
    }

    /// Equivalent to `Node::progn` but `map` except:
    /// - `map` takes a `bool`
    /// - `bool == false` unless on the last element
    ///   where `bool == true`
    pub fn progn_then<F>(&self, mut map: F) -> Err<Obj> 
    where 
        F: FnMut(&'a RcCell<Obj>, bool) -> Err<Obj> 
    {        
        let bounds = self
            .len()
            .checked_sub(1)
            .unwrap_or(0);  

        for i in 0..bounds {
            map(self.get_cell(i)?, false)?;
        }

        map(self.get_cell(bounds)?, true)
    }

    /// Evaluates each element, returning the
    /// evaluation of the last element within a scope
    /// 
    /// ## Example
    /// ```
    /// // equivalent to "let"
    /// (set a nil)
    /// (set b 101)
    /// 
    /// (let (a 10 
    ///       b 20)
    ///     (set x (+ a b)))
    /// 
    /// (assert-eq a nil)
    /// (assert-eq b 101)
    /// (assert-eq x 30)
    /// ```
    pub fn progn_scoped<'b, 'c, I1, I2>(&self, env: &Env, params: I1, args: I2) -> Err<Obj>     
    where 
        I1: Iterator<Item = &'b RcCell<Obj>> + Clone, 
        I2: Iterator<Item = &'c RcCell<Obj>> + Clone,
    {   
        let prev: Node = params
            .clone()
            .map(|obj| env.eval(obj.as_ref()))
            .collect::<Err<_>>()?;   
        
        for (param, arg) in params.clone().zip(args.clone()) { 
            param
                .as_mut()
                .deref_mut()
                .assign(&env.eval(arg.as_ref())?);
        }

        let res = self.progn(|obj| env.eval(obj.as_ref()));

        for (param, val) in params.zip(prev.iter()) {   
            param
                .as_mut()
                .deref_mut()
                .assign(&env.eval(val.as_ref())?);
        }

        res
    }

    pub fn progn_macro<'b, 'c, I1, I2>(&self, env: &Env, params: I1, args: I2) -> Err<Obj>     
    where 
        I1: Iterator<Item = &'b RcCell<Obj>> + Clone, 
        I2: Iterator<Item = &'c RcCell<Obj>> + Clone,
    {   
        let prev: Node = params
            .clone()
            .map(|obj| env.eval(obj.as_ref()))
            .collect::<Err<_>>()?;   
        
        for (param, arg) in params.clone().zip(args.clone()) { 
            param
                .as_mut()
                .deref_mut()
                .assign(&arg.as_ref());
        }

        let res = self.progn(|obj| env.eval(obj.as_ref()));

        for (param, val) in params.zip(prev.iter()) {   
            param
                .as_mut()
                .deref_mut()
                .assign(&env.eval(val.as_ref())?);
        }

        res
    }
}