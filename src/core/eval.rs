
use std::{
    ops::Deref, 
    io::{self, Write},
    time::{Instant, Duration}
};

use super::{
    obj::Obj::{*, self},
    err::Err,
    env::Env,
    parse::Parser,
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
                match self.eval(node.get(0)?.as_ref())? {
                    FnNative(nat) => nat.call(self, node.iter_from(1)),
                    FnBridge(brg) => brg.call(self, node.iter_from(1)),
                    
                    _ => Ok(node.evaled(self)?.into_obj())           
                }          
            }

            Sym(sym) => Ok(sym.as_ref().clone()),
            
            _ => Ok(obj.clone())
        }
    }

    pub fn run(&self) -> Err<Obj> {
        self.prelude().as_ref().run(self)
    }

    pub fn run_module(&self, name: &str) -> Err<Obj> {
        self.module(name)?.as_ref().run(self)
    }

    pub fn run_repl(&self) -> Err<Obj> {
        let mut res = Obj::Nil();
        let mut time = Duration::new(0, 0);
        let mut line = 0;

        loop {
            // print!("{}>> ", line);
            // io::stdout().flush()?;
            // line += 1;

            // let mut buf = String::new();
            // io::stdin().read_line(&mut buf)?;
            // let buf = buf.trim().to_string();

            // if buf.len() < 2 || &buf[0..2] != "--" {
            //     Parser::new_repl(self, buf)?;

            //     let start = Instant::now();

            //     res = self.repl().as_ref().run(self)?;

            //     time = start.elapsed();

            //     println!("{}", res.to_string(self));
            // }
            // else {
            //     match &buf[2..] {
            //         "help" => {
            //             todo!()
            //         }
            //         "quit" => {
            //             println!("quitting...");
            //             break
            //         }
            //         "time" => {
            //             println!("{:?}", time);
            //             io::stdout().flush()?;
            //             continue
            //         }
            //         _ => ()
            //     }
            // }     
        }

        Ok(res)
    }
}