use std::collections::HashMap;
use super::fun::Bridge;
use super::obj::Obj;
use super::rc_cell::RcCell;
use super::err::Err;
use super::type_id::{TypeId, Primitive};

// TODO: work on namespaces - module tree qualifiers

pub trait SymStream {

}

/// `Jester-rs` Environment struct
#[derive(Clone)]
pub struct Env {
    symbols: HashMap<String, RcCell<Obj>>,
}

impl Default for Env {
    /// Returns `Env` with no initial symbols
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
            
        Ok(env)
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

    pub fn add_primitive<T: Primitive>(&mut self, sym: &str, prim: T) -> RcCell<Obj> {
        self.add_sym(sym, prim.into_obj())
    }

    pub fn add_bridge(&mut self, sym: &str, bridge: Bridge) -> RcCell<Obj> {
        let obj = Obj::new_bridge(bridge);
        self.add_sym(sym, obj)
    }
}