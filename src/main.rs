mod core;
mod prelude;

use crate::core::{
    env::Env,
    err::Err,
};

fn main() -> Err {
    let mut env = Env::new()?;
    env.repl()
}
