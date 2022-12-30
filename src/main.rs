mod core;
mod prelude;

use crate::core::{
    env::Env,
    err::Err,
};

fn main() -> Err<()>{
    let mut env = Env::new()?;

    env.add_from_string(
    "
    (println 10 15 20)
    (print 99)
    (println 11)
    (+ 1 2 3)
    "
    )?;

    env.repl()
}
