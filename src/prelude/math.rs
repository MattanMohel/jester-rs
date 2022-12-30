use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj, type_id::TypeId,
};

impl Env {
    pub fn math_lib(&mut self) {

        self.add_primitive("E",  2.71828);
        
        self.add_primitive("PI", 3.14159);        

        self.add_bridge("+", |env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.add(env.eval(rst.as_ref())?)?;
            }

            Ok(fst)
        });

        self.add_bridge("-", |env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;

            if args.len() == 1 {
                fst.neg()?;
            }

            for rst in args.skip(1) {
                fst.sub(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("*", |env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.mul(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("/", |env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.div(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("%", |env, args| {
            let mut fst = env.eval(args.get_ref(0)?)?;
            for rst in args.skip(1) {
                fst.modulos(env.eval(rst.as_ref())?)?;
            }
            
            Ok(fst)
        });

        self.add_bridge("=", |env, node| {
            let res = env
                .eval(node.get(0)?.as_ref())?
                .eq(&env.eval(node.get(1)?.as_ref())?)?;

            Ok(res.as_obj())
        });

        self.add_bridge("<=", |env, node| {
            let res = env
                .eval(node.get(0)?.as_ref())?
                .le_eq(&env.eval(node.get(1)?.as_ref())?)?;
                
            Ok(res.as_obj())
        });

        self.add_bridge(">=", |env, node| {
            let res = !env
                .eval(node.get(0)?.as_ref())?
                .le(&env.eval(node.get(1)?.as_ref())?)?;
                
            Ok(res.as_obj())
        });

        self.add_bridge("<", |env, node| {
            let res = env
                .eval(node.get(0)?.as_ref())?
                .le(&env.eval(node.get(1)?.as_ref())?)?;
                
            Ok(res.as_obj())
        });

        self.add_bridge(">", |env, node| {
            let res = !env
                .eval(node.get(0)?.as_ref())?
                .le_eq(&env.eval(node.get(1)?.as_ref())?)?;
                
            Ok(res.as_obj())
        });
    }
}