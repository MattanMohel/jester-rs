use std::collections::HashMap;

use super::{err::{Err, ErrType::*, AsResult}, rc_cell::RcCell, object::Obj, id::Id};

const PRELUDE: &str = "prelude";

pub trait Context: PartialEq {
    /// Returns whether context contains symbol
    fn has_sym(&self, sym: &String) -> bool;
    /// Returns symbol, erroring if 
    /// context doesn't contain the symbol
    fn get_sym(&self, sym: &String) -> Err<RcCell<Obj>>;
    /// Adds symbol, erroring if 
    /// context already contains the symbol
    fn add_sym(&mut self, sym: &str, val: Obj) -> Err;
    /// Returns whether context contains
    /// module import or if sym import is self
    fn has_import(&self, sym: &String) -> bool;
    /// Adds module and all its imports as self imports
    fn add_import(&mut self, pkg: RcCell<Mod>);
}

pub struct Mod {
    symbols: HashMap<String, RcCell<Obj>>,
    imports: HashMap<String, RcCell<Mod>>,
    name: String,
    id: Id
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Context for Mod {
    fn has_sym(&self, sym: &String) -> bool {
        self.symbols.contains_key(sym) ||
        self.imports.values().any(|imp| imp.as_ref().symbols.contains_key(sym))
    }

    fn get_sym(&self, sym: &String) -> Err<RcCell<Obj>> {
        if let Some(val) = self.symbols.get(sym) {
            return Ok(val.clone())
        }
        self.imports
            .iter()
            .find_map(|(_, imp)| { 
                imp.as_ref().symbols
                    .get(sym)
                    .cloned()
            })
            .ok_or(NonSym)     
    }

    fn add_sym(&mut self, sym: &str, val: Obj) -> Err {
        let sym = sym.to_string();

        self.has_sym(&sym).ok_or(DupSym)?;
        self.symbols.insert(sym, RcCell::from(val));
        Ok(())
    }

    fn has_import(&self, sym: &String) -> bool {
        self.imports.contains_key(sym)
    }

    fn add_import(&mut self, pkg: RcCell<Mod>) {
        if !(self.id == pkg.as_ref().id || self.has_import(&pkg.as_ref().name)) {
            self.imports.insert(pkg.as_ref().name.clone(), pkg.clone());

            for imp in pkg.as_ref().imports.values() {
                self.add_import(imp.clone());
            }
        }
    }
}

impl Mod {
    pub fn new(name: String, prelude: RcCell<Mod>) -> Self {
        let mut module = 
            Self { 
                symbols: HashMap::new(), 
                imports: HashMap::new(), 
                name: name, id: 
                Id::new() 
            };
        module.add_import(prelude);
        module
    }

    pub fn new_prelude() -> Self {
        Self { 
            symbols: HashMap::new(), 
            imports: HashMap::new(), 
            name: PRELUDE.to_string(), 
            id: Id::new() 
        }
    }
}

pub struct Env {
    prelude: RcCell<Mod>,
    imports: HashMap<String, RcCell<Mod>>,
    id: Id
}

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Context for Env {
    fn has_sym(&self, sym: &String) -> bool {
        self.prelude.as_ref().symbols.contains_key(sym) ||
        self.imports.values().any(|imp| imp.as_ref().symbols.contains_key(sym))
    }

    fn get_sym(&self, sym: &String) -> Err<RcCell<Obj>> {
        if let Some(val) = self.prelude.as_ref().symbols.get(sym) {
            return Ok(val.clone())
        }
        self.imports
            .iter()
            .find_map(|(_, imp)| { 
                imp.as_ref().symbols
                    .get(sym)
                    .cloned()
            })
            .ok_or(NonSym)     
    }

    fn add_sym(&mut self, sym: &str, val: Obj) -> Err {
        let sym = sym.to_string();

        self.has_sym(&sym).ok_or(DupSym)?;
        self.prelude.as_mut().symbols.insert(sym, RcCell::from(val));
        Ok(())
    }

    fn has_import(&self, sym: &String) -> bool {
        self.imports.contains_key(sym)
    }

    fn add_import(&mut self, pkg: RcCell<Mod>) {
        if !(self.id == pkg.as_ref().id || self.has_import(&pkg.as_ref().name)) {
            self.imports.insert(pkg.as_ref().name.clone(), pkg.clone());

            for imp in pkg.as_ref().imports.values() {
                self.add_import(imp.clone());
            }
        }
    }
}

impl Env {
    pub fn new() -> Self {
        Self { 
            prelude: RcCell::from(Mod::new_prelude()), 
            imports: HashMap::new(), id: Id::new() 
        }

        // add prelude symbols
    }
}
