use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj,
};

impl Env {
    pub fn math_lib(&mut self) {
        self.add_primitive("E",  2.71828);
        self.add_primitive("PI", 3.14159);        

        self.add_bridge("+", |env, args| 
        {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.add(env.eval(rst.as_ref())?)?;
            }

            Ok(fst)
        });

        self.add_bridge("-", |env, args| 
        {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.sub(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("*", |env, args| 
        {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.mul(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("/", |env, args| 
        {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.div(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("%", |env, args| 
        {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.modulos(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });
    }
}