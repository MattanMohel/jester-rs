use crate::core::{env::Env, err::ErrType::RuntimeAssert, type_id::TypeId};

impl Env {
    pub fn io_lib(&mut self) {

        self.add_bridge("print", |env, args| {
            args.progn(|obj| {
                let res = env.eval(obj.as_ref())?;
                print!("{}", res.as_string(env));
                Ok(res)
            })
        });
        
        self.add_bridge("println", |env, args| {
            args.progn_then(|obj, last| {
                let res = env.eval(obj.as_ref())?;

                if !last {
                    print!("{}", res.as_string(env));
                }
                else {
                    println!("{}", res.as_string(env));
                }

                Ok(res)
            })
        });

        self.add_bridge("format", |env, args| {
            const PAT: &str = "{}";

            let mut str = env
                .eval(args.get(0)?)?
                .is_string()?
                .clone();

            for i in 0..args.len()-1 {
                match str.find(PAT) {
                    Some(pos) => {     
                        let to = env
                            .eval(args.get(i+1)?)?
                            .as_string(env);
                        let beg = pos + PAT.len();
                        str = str[0..beg].replace(PAT, &to) + &str[beg..]; 
                    }

                    None => return Err(RuntimeAssert)
                }
            }

            Ok(str.as_obj())
        });
    }
}