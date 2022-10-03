use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj,
};

impl Env {
    pub fn std_lib(&mut self) -> Err {
        self.add_sym("set", Obj::new_bridge(|env, args| {
            let val = env.eval(args.get(1)?.as_ref())?;
            args.get(0)?.as_mut().set(&val)?;

            Ok(val)
        }))?;

        Ok(())
    }
}