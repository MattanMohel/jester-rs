use std::collections::HashMap;

use super::{
    obj::Obj,
    err::Err,
    fun::Bridge, 
    rc_cell::RcCell,
    type_id::Primitive, id::Id
};

// TODO: work on namespaces - module tree qualifiers



/// `Jester-rs` Environment struct
#[derive(Clone)]
pub struct Env {
    symbols: HashMap<String, RcCell<Obj>>,
}

impl Default for Env {
    fn default() -> Self {
        Self { 
            symbols: HashMap::new() 
        }
    }
}

impl Env {
    pub fn new() -> Err<Self> {
        let mut env = Self::default();
        env.math_lib();
        env.std_lib();
        env.io_lib();
        env.list_lib();
            
        Ok(env)
    }

    pub fn unique_sym() -> String {
        format!("G#{}", Id::next_id())
    }   

    pub fn add_sym(&mut self, sym: &str, val: Obj) -> RcCell<Obj> {
        let pop = self.symbols.insert(sym.to_string(), RcCell::from(val));
        assert!(pop.is_none(), "\"{}\" already exists!", sym);
        self.symbols[sym].clone()
    }

    pub fn get_sym(&self, sym: &str) -> Option<RcCell<Obj>> {
        self.symbols
            .get(sym)
            .cloned()
    }

    pub fn has_sym(&self, sym: &str) -> bool {
        self.symbols
            .keys()
            .any(|rhs| sym == rhs)
    } 

    pub fn get_sym_id(&self, obj: &RcCell<Obj>) -> Option<String> {
        self.symbols
            .iter()
            .find_map(|rhs| {
                if obj.raw_eq(rhs.1) {
                    Some(rhs.0.to_uppercase())
                }
                else {
                    None
                }
            })
            
    }

    pub fn add_primitive<T: Primitive>(&mut self, sym: &str, prim: T) -> RcCell<Obj> {
        self.add_sym(sym, prim.as_obj())
    }

    pub fn add_bridge(&mut self, sym: &str, bridge: Bridge) -> RcCell<Obj> {
        let obj = Obj::new_bridge(sym.to_string(), bridge);
        self.add_sym(sym, obj)
    }

    pub unsafe fn gen_sym(&self, obj: Obj) -> RcCell<Obj> {   
        // coerce self mutability   
        let ptr = (self as *const Self) as *mut Self;
            
        match ptr.as_mut() {
            Some(env) => {
                let sym = Env::unique_sym();
                env.add_sym(sym.as_str(), obj)
            }
            None => panic!("environment not initialized!")
        } 
    }
}