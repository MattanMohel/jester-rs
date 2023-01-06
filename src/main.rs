mod core;
mod prelude;

use crate::core::{
    env::Env,
    err::Err,
};

/// TODO: 
/// stop functions like 'nth' 'len' 'take' from evaluating their lists --> want (len (+ 1 2 3)) == 4
/// make gen-sym symbols for inlined lists --> easier evaluation pattern matching

const INTRO: &str = "Welcome to Jester Script, the Rust-Lisp Scripting Langauge!";

// Add Env Builder

// give env a working director (script folder)

// Env::default()
//     .math_lib()
//     .io_lib()
//     .add_from_string(
//     "
//        (defun test () nil) 
//     ")
//     .build();

fn main() -> Err {
    println!("{}\n", INTRO);

    let mut env = Env::new()?;

    // env.add_from_file("src/scripts/test.lsp")?;

    env.repl()?;

    Ok(())
}
