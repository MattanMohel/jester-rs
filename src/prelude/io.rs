use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj,
};

impl Env {
    pub fn io_lib(&mut self) {

        self.add_bridge("print", |env, args| {
            args.progn(|obj| {
                let res = env.eval(obj.as_ref())?;
                print!("{}", res.to_string(env));
                Ok(res)
            })
        });
        
        self.add_bridge("println", |env, args| {
            args.progn_then(|obj, last| {
                let res = env.eval(obj.as_ref())?;

                if !last {
                    print!("{}", res.to_string(env));
                }
                else {
                    println!("{}", res.to_string(env));
                }

                Ok(res)
            })
        });
    }
}