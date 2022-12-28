mod core;
mod prelude;

use crate::core::{
    env::Env,
    err::Err,
};

fn main() -> Err<()>{
    let mut env = Env::new()?;

    // // let node = vec![ RcCell::from(Obj::I32(0)), RcCell::from(Obj::I32(1)), RcCell::from(Obj::I32(2)) ];

    // // let obj = Obj::Lst(Node::from(node));

    // // println!("{}", obj.to_string(&env));

    // let prelude = env.prelude();

    // println!("{}", prelude.as_ref().to_string());

    // let res = env.add_from_string(
    // "
    // (set x -1_234_567) 
    // (set y #b10011)
    // (+ x y)
    // ");

    // let inner = res.as_raw().borrow().clone();
    // let inner = inner.is_list().unwrap();
    // let result = inner.iter().progn(|obj, _| env.eval(obj.as_ref()));
    
    // println!("{}", result.unwrap().to_string(&env));
    // env.add_mod("test", Mod::from(p.module()))?;

    // let m = env.module("test")?;

    // println!("{}", m.as_ref().to_string());

    // let eval = env.run_module("test")?;

    // println!("eval: {}", eval.to_string(&env));

    // env.run_repl()?;


    Ok(())
}
