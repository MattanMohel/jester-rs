mod core;
mod prelude;

use crate::core::{
    env::Env,
    err::Err,
};

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
    let mut env = Env::new()?;

    env.add_from_file("src/scripts/test.lsp")?;

    env.repl()?;

    Ok(())
}
