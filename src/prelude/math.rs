use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj,
};

impl Env {
    pub fn math_lib(&mut self) -> Err {
        self.add_sym("PI", Obj::new_value(3.14159))?;
        self.add_sym("E", Obj::new_value(2.71828))?;


        self.add_sym("+", Obj::new_bridge(|env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            for rst in args.skip(1) {
                fst.add(env.eval(rst.as_ref())?)?;
            }

            Ok(fst)
        }))?;

        self.add_sym("-", Obj::new_bridge(|env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            for rst in args.skip(1) {
                fst.sub(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        }))?;

        self.add_sym("*", Obj::new_bridge(|env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            for rst in args.skip(1) {
                fst.mul(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        }))?;

        self.add_sym("/", Obj::new_bridge(|env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            for rst in args.skip(1) {
                fst.div(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        }))?;

        self.add_sym("%", Obj::new_bridge(|env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            for rst in args.skip(1) {
                fst.modulos(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        }))?;

        Ok(())
    }
}