use std::io::Write;
use std::time::{Duration, Instant};
use super::err::Err;
use super::env::Env;

impl Env {
    pub fn repl(&mut self) -> Err<()> {
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
                    println!("quit REPL...");
                    break;
                },
                "--time" => {
                    println!("completed in: {:?}", time);
                    continue;
                },
                _ => ()
            }

            let start = Instant::now();
            let res = self.add_from_string(&input.trim().to_string())?;
            time = start.elapsed();

            println!("{}", res.to_string(self));
        }

        Ok(())
    }
}