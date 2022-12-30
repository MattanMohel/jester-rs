
use std::{
    ops::Deref, 
    io::{self, Write},
    time::{Instant, Duration}
};

use super::{
    obj::Obj::{*, self},
    err::Err,
    env::Env,
    fun::Callable, 
    type_id::TypeId
};

impl Env {
    pub fn eval<T>(&self, obj: T) -> Err<Obj> 
    where   
        T: Deref<Target=Obj>
    {    
        match obj.deref() {
            Lst(node) if !node.is_empty() => {
                let fst = node.get(0).unwrap();    
                match self.eval(fst.as_ref())? {
                    Native(f) => f.call(self, node.iter_from(1)),
                    Bridge(f) => f.call(self, node.iter_from(1)),
                    
                    _ => {
                        Ok(node
                            .iter()
                            .evaled(self)?
                            .as_obj()
                        )
                    }           
                }          
            }

            Sym(sym) => Ok(sym.clone_inner()),  
             
            _ => Ok(obj.clone())
        }
    }

    pub fn run(&self) -> Err<Obj> {
        // self.prelude().as_ref().run(self)
        todo!()
    }

    pub fn run_module(&self, name: &str) -> Err<Obj> {
        // self.module(name)?.as_ref().run(self)
        todo!()
    }
}