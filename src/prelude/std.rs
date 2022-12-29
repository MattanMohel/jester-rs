use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj, type_id::TypeId,
};

impl Env {
    pub fn std_lib(&mut self) {
        
        self.add_bridge("set", |env, args| {
            let val = env.eval(args.get(1)?.as_ref())?;
            args.get(0)?.as_mut().assign(&val);

            Ok(val)
        });

        self.add_bridge("quote", |_, args| {
            Ok(args.get(0)?.as_ref().clone())
        });
    }
}