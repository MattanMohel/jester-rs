use super::{
    obj::Obj::{*, self},
    err::Err,
    env::Env,
    fun::Callable, 
    type_id::TypeId,
    node::NodeIter
};

impl Env {
    pub fn eval(&self, obj: &Obj) -> Err<Obj> {    
        match obj {      
            Lst(node) if !node.is_empty() => {       
                match node.get(0).unwrap() {
                    Sym(sym) => {
                        match sym.as_ref() {
                            Native(f) => f.call(self, node.iter_from(1)),
                            Bridge(f) => f.call(self, node.iter_from(1)),
                            Macro(f)  => f.call(self, node.iter_from(1)),
                            _ => Ok(node.evaled(self)?.as_obj())
                        }
                    }
                    _ => Ok(node.evaled(self)?.as_obj())
                }     
            }

            Sym(sym) => Ok(sym.clone_inner()),
         
            _ => Ok(obj.clone())
        }
    }

    pub fn eval_args<const L: usize>(&self, indices: [usize; L], args: NodeIter) -> Err<[Obj; L]> {
        const NIL: Obj = Obj::Nil(());
        let mut arr = [NIL; L];
        
        for (i, index) in indices.iter().enumerate() {
            arr[i] = args.get(*index)?.eval(self)?;
        }

        Ok(arr)
    }
}