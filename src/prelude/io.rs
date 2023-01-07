use crate::core::{env::Env, err::ErrType::RuntimeAssert, type_id::TypeId};

impl Env {
    pub fn io_lib(&mut self) {

        // (print ..items)
        self.add_bridge("print", |env, args| {
            args.progn(|obj| {
                let item = obj.as_ref().eval(env)?;

                print!("{}", item.as_string(env));

                Ok(item)
            })
        });
        
        // (println ..items)
        self.add_bridge("println", |env, args| {
            args.progn_then(|obj, last| {
                let item = obj.as_ref().eval(env)?;

                if !last {
                    print!("{}", item.as_string(env));
                }
                else {
                    println!("{}", item.as_string(env));
                }

                Ok(item)
            })
        });

        // (format source ..items)
        self.add_bridge("format", |env, args| {
            const PAT: &str = "{}";

            let mut source = args
                .get(0)?
                .eval(env)?
                .is_string()?
                .clone();

            for i in 0..args.len()-1 {
                match source.find(PAT) {
                    Some(pos) => {     
                        let format = args
                            .get(i+1)?
                            .eval(env)?
                            .as_string(env);

                        let beg = pos + PAT.len();
                        source = source[0..beg].replace(PAT, &format) + &source[beg..]; 
                    }

                    None => return Err(RuntimeAssert)
                }
            }

            Ok(source.as_obj())
        });
    }
}