use std::{collections::HashMap, ops::{Deref, DerefMut}};

use super::{
    id::Id,
    obj::Obj, 
    parse::Parser,
    rc_cell::RcCell, 
    err::{Err, ErrType::*, AsResult}
};

/// Name of `prelude` Module
const PRELUDE: &str = "prelude";
/// Name of `REPL` Module
const REPL: &str = "REPL";
/// HashMap alias for symbol-value dictionary
type Dict<T> = HashMap<String, T>;

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Environment Struct
pub struct Env {
    modules: Dict<RcCell<Mod>>,
    id: Id
}

impl Env {
    /// Creates Environment with a default `prelude` Module
    /// ## Error 
    /// * creating `prelude` : `Err(_)`
    pub fn new() -> Err<Self> {
        let mut env = {
            Self { 
                modules: Dict::new(), 
                id: Id::new() 
            }
        };

        env.add_mod(PRELUDE, Mod::new_prelude())?;
        env.add_mod(REPL, Mod::new(REPL.to_string()))?;

        // prelude imports
        env.math_lib()?;
        env.std_lib()?;
        
        Ok(env)
    }

    pub fn add_sym(&mut self, sym: &str, val: Obj) -> Err<RcCell<Obj>> {
        self.prelude().as_mut().add_sym(sym.to_string(), val)
    }

    /// Appends Module with name `sym` and `prelude` import
    /// ## Error
    /// * duplicate `sym` : `Err(DupMod)`
    pub fn new_mod(&mut self, sym: &str) -> Err {
        self.add_mod(sym, Mod::new(sym.to_string()))
    }

    /// Appends Module with name `sym`, `prelude` import, and symbols defined in `parser`
    /// ## Error
    /// * duplicate `sym` : `Err(DupMod)`
    /// * duplicate symbols in `parser` : `Err(DupSym)`
    pub fn add_mod(&mut self, sym: &str, mut module: Mod) -> Err {
        (!self.modules.keys().any(|key| *key == sym)).ok_or(DupSym)?;

        // if added module isn't the prelude
        if self.modules.len() != 0 {
            module.import(self.prelude());
        }

        self.modules.insert(sym.to_string(), RcCell::from(module));
        Ok(())
    }

    /// Returns `prelude` Module
    pub fn prelude(&self) -> RcCell<Mod> {
        self.modules 
            .get(&PRELUDE.to_string())
            .unwrap()
            .clone()
    }

    /// Returns `REPL` Module
    pub fn repl(&self) -> RcCell<Mod> {
        self.modules 
            .get(&REPL.to_string())
            .unwrap()
            .clone()
    }

    /// Returns Module with name `sym` from `modules`
    /// ## Error
    /// * `sym` not a module : `Err(NonMod)`
    pub fn module(&self, sym: &str) -> Err<RcCell<Mod>> {
        self.modules
            .iter()
            .find_map(|(k, v)| if k == sym { Some(v.clone()) } else { None })
            .ok_or(NonMod)
    }

    /// Returns if Environment contains Module `sym`
    pub fn has_module(&self, sym: &str) -> bool {
        self.modules
            .keys()
            .any(|k| k == sym)
    }

    pub fn sym(&self, sym: &str) -> Err<RcCell<Obj>> {
        self.modules
            .values()
            .find_map(|module| module.as_ref().sym(&sym.to_string()).ok())
            .ok_or(NonSym)            
    }

    pub fn cell_sym(&self, sym: &RcCell<Obj>) -> Err<String> {
        self.modules
            .values()
            .find_map(|module| module.as_ref().sym_raw(&sym).ok())
            .ok_or(NonSym)            
    }
}

impl PartialEq for Mod {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Clone for Mod {
    fn clone(&self) -> Self {
        Self { 
            symbols: self.symbols.clone(),
            imports: self.imports.clone(),
            exec: self.exec.clone(),
            name: self.name.clone(),
            id: Id::new()
        }
    }
}

impl From<Parser> for Mod {
    fn from(parser: Parser) -> Self {
        parser.into()
    }
}

/// Module Struct
pub struct Mod {
    symbols: Dict<RcCell<Obj>>,
    imports: Dict<RcCell<Mod>>,
    exec: Obj,
    name: String,
    id: Id
}

impl Mod {
    pub fn to_string(&self) -> String {    
        let symbols = {
            self.symbols
                .iter()
                .fold(String::new(), |acc, (sym, val)| 
                    acc + &format!("\t\t\"{}\": {}\n", sym, val.as_ref().to_type_string()))
        };

        let imports = {
            self.imports
                .keys()
                .fold(String::new(), |acc, name|
                    acc + &format!("\t\t\"{}\"\n", name))
        };

        format!("\"{}\" {{\n\tsymbols {{\n{}\t}}\n\timports {{\n{}\t}}\n}}", self.name, symbols, imports)
    }

    /// Creates Module with `sym` and imports `prelude`
    pub(crate) fn new(name: String) -> Self {
        Self { 
            symbols: Dict::new(), 
            imports: Dict::new(), 
            exec: Obj::Nil(),
            id: Id::new(),
            name
        }
    }

    /// Creates a `prelude` Module
    pub fn new_prelude() -> Self {
        Self { 
            symbols: Dict::new(), 
            imports: Dict::new(), 
            exec: Obj::Nil(),
            name: PRELUDE.to_string(),
            id: Id::new() 
        }
    }

    pub fn add_exec(&mut self, exec: Obj) {
        self.exec = exec;
    }

    /// Returns if `sym` is a symbol contained within the module's `symbols` or `imports`
    pub fn has_sym(&self, sym: &String) -> bool {
        self.symbols.contains_key(sym) ||
        self.imports.values().any(|import| import.as_ref().symbols.contains_key(sym))
    }

    /// Adds `sym` with `val` to `symbols`
    /// ## Error
    /// * Module already contains `sym` :  `Err(DupSym)`
    pub fn add_sym(&mut self, sym: String, val: Obj) -> Err<RcCell<Obj>> {        
        (!self.has_sym(&sym)).ok_or(DupSym)?;
        self.symbols.insert(sym.clone(), RcCell::from(val));
        self.sym(&sym)
    }

    /// Returns `sym` `RcCell<Obj>` value from Module
    /// ## Error
    /// * `sym` not found : `Err(NonSym)`
    pub fn sym(&self, sym: &String) -> Err<RcCell<Obj>> {
        if let Some(val) = self.symbols.get(sym) {
            return Ok(val.clone())
        }
        self.imports
            .values()
            .find_map(|import| import.as_ref().symbols.get(sym).cloned())
            .ok_or(NonSym)     
    }

    /// TODO
    pub fn sym_raw(&self, sym: &RcCell<Obj>) -> Err<String> {
        if let Some((key, val)) = self.symbols.iter().find(|(key, val)| *val == sym) {
            return Ok(key.clone())
        }
        self.imports
            .values()
            .find_map(|import| import.as_ref().symbols.iter().find_map(|(key, val)| if val == sym { Some(key) } else { None }).cloned())
            .ok_or(NonSym)     
    }

    /// Adds `import` and all its imports to the Module
    /// ## Note
    /// if `import` or any of its imports are already 
    /// contained in the Module, they will be skipped
    pub fn import(&mut self, import: RcCell<Mod>) {
        if self.id != import.as_ref().id && !self.imports.contains_key(&import.as_ref().name) {
            self.imports.insert(import.as_ref().name.clone(), import.clone());     
            for import in import.as_ref().imports.values() {
                self.import(import.clone());
            }
        }
    }

    /// Runs Module Executable
    pub fn run(&self, env: &Env) -> Err<Obj> {
        self
            .exec()
            .is_list()?
            .iter()
            .progn(|obj, _| env.eval(obj.as_ref()))
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn exec(&self) -> &Obj {
        &self.exec
    }
}
