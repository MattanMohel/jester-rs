use std::{
    collections::HashMap, 
    io::Write, 
    time::{Duration, Instant}
};

use super::{
    obj::Obj,
    err::Err,
    fun::Bridge, 
    rc_cell::RcCell,
    type_id::Primitive, id::Id
};

// TODO: work on namespaces - module tree qualifiers

/// Path to native prelude definitions 
const PRELUDE_PATH: &str = "src/scripts/prelude.lsp";
/// String introduction for `REPL` mode
const REPL_HEADER: &str = "Welcome to Jester Script, the Rust-Lisp Scripting Langauge!\nDeveloped by Mattan Mohel, 2021-2023";


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
        env.add_from_file(PRELUDE_PATH)?;
            
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

    pub fn gen_sym(&mut self, obj: Obj) -> RcCell<Obj> {   
        let sym = Env::unique_sym();
        self.add_sym(sym.as_str(), obj)
    }

    pub unsafe fn gen_sym_runtime(&self, obj: Obj) -> RcCell<Obj> {   
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
    
    pub fn repl(&mut self) -> Err {
        println!("{}", REPL_HEADER);

        let mut time = Duration::new(0, 0);

        loop {
            print!(">> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim() {
                "--help" => {
                    unimplemented!()
                }
                "--quit" => {
                    println!("quitting...");
                    break;
                },
                "--time" => {
                    println!("completed in: {:?}...", time);
                    continue;
                },
                _ => ()
            }

            let start = Instant::now();
            let res = self.add_from_string(&input.trim().to_string())?;
            time = start.elapsed();

            println!("{}", res.display(self));
        }

        Ok(())
    }
}