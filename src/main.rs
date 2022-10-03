mod core;
mod prelude;

use crate::core::{
    env::{Env, Mod},
    err::Err,
    parse::Parser
};

fn main() -> Err<()>{
    let mut env = Env::new()?;

    // let node = vec![ RcCell::from(Obj::I32(0)), RcCell::from(Obj::I32(1)), RcCell::from(Obj::I32(2)) ];

    // let obj = Obj::Lst(Node::from(node));

    // println!("{}", obj.to_string(&env));

    let prelude = env.prelude();

    println!("{}", prelude.as_ref().to_string());

    let p = Parser::from_src(&env, "test", 
    "
    
    (set x 10) 
    (set x (* 101 x))
    x
    
    ")?;

    env.add_mod("test", Mod::from(p))?;

    let m = env.module("test")?;

    println!("{}", m.as_ref().to_string());

    let eval = env.run_module("test")?;

    println!("eval: {}", eval.to_string(&env));

    env.run_repl()?;


    Ok(())
}
