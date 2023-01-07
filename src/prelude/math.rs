use crate::core::{
    env::Env,
    type_id::TypeId,
};

impl Env {
    pub fn math_lib(&mut self) {

        // constant E = 2.718
        self.add_primitive("E",  2.71828);
        
        // constant PI = 3.141
        self.add_primitive("Pi", 3.14159);        

        // (+ first ..rest)
        self.add_bridge("+", |env, args| {
            let mut first = args.get(0)?.eval(env)?;

            for rest in args.skip(1) {
                first.add(rest.as_ref().eval(env)?)?;
            }

            Ok(first)
        });

        // (- first ..rest)
        self.add_bridge("-", |env, args| {
            let mut first = args.get(0)?.eval(env)?;

            if args.len() == 1 {
                first.neg()?;
            }

            for rest in args.skip(1) {
                first.sub(rest.as_ref().eval(env)?)?;
            }
            
            Ok(first)
        });

        // (* first ..rest)
        self.add_bridge("*", |env, args| {
            let mut first = args.get(0)?.eval(env)?;

            for rest in args.skip(1) {
                first.mul(rest.as_ref().eval(env)?)?;
            }
            
            Ok(first)
        });

        // (/ first ..rest)
        self.add_bridge("/", |env, args| {
            let mut first = args.get(0)?.eval(env)?;

            for rest in args.skip(1) {
                first.div(rest.as_ref().eval(env)?)?;
            }
            
            Ok(first)
        });

        // (% first ..rest)
        self.add_bridge("%", |env, args| {
            let mut first = args.get(0)?.eval(env)?;

            for rest in args.skip(1) {
                first.modulos(rest.as_ref().eval(env)?)?;
            }
            
            Ok(first)
        });

        // (= lhs rhs)
        self.add_bridge("=", |env, args| {
            let lhs = args
                .get(0)?
                .eval(env)?
                .eq(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });

        // (!= lhs rhs)
        self.add_bridge("!=", |env, args| {
            let lhs = !args
                .get(0)?
                .eval(env)?
                .eq(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });

        // (<= lhs rhs)
        self.add_bridge("<=", |env, args| {
            let lhs = args
                .get(0)?
                .eval(env)?
                .le_eq(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });

        // (>= lhs rhs)
        self.add_bridge(">=", |env, args| {
            let lhs = !args
                .get(0)?
                .eval(env)?
                .le(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });

        // (< lhs rhs)
        self.add_bridge("<", |env, args| {
            let lhs = args
                .get(0)?
                .eval(env)?
                .le(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });

        // (> lhs rhs)
        self.add_bridge(">", |env, args| {
            let lhs = !args
                .get(0)?
                .eval(env)?
                .le_eq(&args.get(1)?.eval(env)?)?;

            Ok(lhs.as_obj())
        });
    }
}