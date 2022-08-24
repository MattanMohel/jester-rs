use std::collections::HashMap;

use super::{
    id::Id,
    object::Obj, 
    parse::Parser,
    rc_cell::RcCell, 

    err::{
        Err,
        AsResult,
        ErrType::*, 
    },
};

/// Name of the deault `prelude` Module
const PRELUDE: &str = "prelude";
/// HashMap alias for a symbol-value dictionary
type Dict<T> = HashMap<String, T>;

/// Structure for storing
/// and organizing symbols
pub struct Mod {
    symbols: Dict<RcCell<Obj>>,
    imports: Dict<RcCell<Mod>>,
    name: String,
    id: Id
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Mod {
    /// Creates a new module with a given name `name` 
    /// and provided default import module `prelude`
    pub(crate) fn new(name: String, prelude: RcCell<Mod>) -> Self {
        let mut module = 
            Self { 
                symbols: Dict::new(), 
                imports: Dict::new(), 
                name: name,
                id: Id::new() 
            };
        module.add_mod(prelude);
        module
    }
    /// Creates a new default `prelude` 
    /// module with no starting imports
    pub fn new_prelude() -> Self {
        Self { 
            symbols: Dict::new(), 
            imports: Dict::new(), 
            name: PRELUDE.to_string(),
            id: Id::new() 
        }
    }
    /// Returns `true` if `sym` is a symbol contained within
    /// the module's `symbols` or `imports`, otherwise returns `false`
    pub fn has_sym(&self, sym: &String) -> bool {
        self.symbols.contains_key(sym) ||
        self.imports.values().any(|imp| imp.as_ref().symbols.contains_key(sym))
    }
    /// Adds symbol `sym` with value `val` to the module `symbols`
    /// - returns `Err(DupSym)` if module already contains 
    ///   symbol `sym` in either its `symbols` or `imports`
    pub fn add_sym(&mut self, sym: &str, val: Obj) -> Err<RcCell<Obj>> {
        let sym = sym.to_string();
        
        self.has_sym(&sym).ok_or(DupSym)?;
        self.symbols.insert(sym.clone(), RcCell::from(val));
        self.sym(&sym)
    }
    /// Searches module's `symbols` and `imports` for 
    /// symbol `sym` returning the `RcCell<Obj>` value
    /// - returns `Err(NonSym)` if `sym` was not found
    ///   in either the module `symbols` or `imports`
    pub fn sym(&self, sym: &String) -> Err<RcCell<Obj>> {
        if let Some(val) = self.symbols.get(sym) {
            return Ok(val.clone())
        }
        self.imports
            .values()
            .find_map(|import| import.as_ref().symbols.get(sym).cloned())
            .ok_or(NonSym)     
    }
    /// Adds `import` and all its imports to the module
    /// - continues recursively for all proceeding
    ///   imports, skipping any duplicate modules
    pub fn add_mod(&mut self, import: RcCell<Mod>) {
        if self.id != import.as_ref().id && !self.imports.contains_key(&import.as_ref().name) {
            self.imports.insert(import.as_ref().name.clone(), import.clone());
            
            for import in import.as_ref().imports.values() {
                self.add_mod(import.clone());
            }
        }
    }
}

/// Structure for storing 
/// and organizing Modules
pub struct Env {
    modules: Dict<RcCell<Mod>>,
    id: Id
}

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Env {
    /// Creates a new Environment with a new default 
    /// `prelude` Module with the specified symbol definitions
    /// - errors if an error is met in either creating 
    ///   `prelude` or in creating prelude definitions
    pub fn new() -> Err<Self> {
        let mut env =
            Self { 
                modules: Dict::new(), id: Id::new() 
            };
        env.new_mod(PRELUDE)?;
        Ok(env)
    }
    /// Creates a new empty Module with name `sym` 
    /// and imports the default `prelude` Module
    /// - `Err(DupMod)` if a Module with name
    ///   `sym` already exists in context
    pub fn new_mod(&mut self, sym: &str) -> Err {
        self.modules.keys().any(|key| *key == sym).ok_or(DupSym)?;
        self.modules.insert(sym.to_string(), RcCell::from(Mod::new(sym.to_string(), self.prelude())));
        Ok(())
    }
    /// Creates a new Module with name `sym`, imports
    /// the default `prelude` Module, and appends the 
    /// symbols and imports defined in Parser `parser`
    /// - `Err(DupMod)` if a Module with name
    ///   `sym` already exists in context
    /// - errors if coinflicting symbols arise from Parser imports
    pub fn create_mod(&mut self, sym: &str, parser: &Parser) -> Err {
        self.modules.keys().any(|key| *key == sym).ok_or(DupSym)?;

        let module = Mod::new(sym.to_string(), self.prelude());



        Ok(())
    }
    /// Returns the default `prelude` Module
    /// - since the creation of `Env` is dependent 
    ///   on the creation of `prelude`, this method
    ///   may not error
    pub fn prelude(&self) -> RcCell<Mod> {
        self.modules 
            .get(&PRELUDE.to_string())
            .unwrap()
            .clone()
    }
    /// Returns Module `sym` from `modules`
    /// - `Err(NonMod)` if `sym` is not a Module
    pub fn module(&self, sym: &str) -> Err<RcCell<Mod>> {
        self.modules
            .iter()
            .find_map(|(k, v)| {
                if k == sym {
                    Some(v.clone())
                }
                else {
                    None
                }
            })
            .ok_or(NonMod)
    }
}
