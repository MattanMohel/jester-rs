use crate::core::{
    env::Env,
    type_id::TypeId, 
    fun::{FnNative, FnMacro},
    err::ErrType::*, obj::Obj
};

impl Env {
    pub fn std_lib(&mut self) {

        self.add_primitive("nil", ());

        self.add_primitive("true", true);

        self.add_primitive("false", false);
        
        self.add_bridge("set", |env, args| {
            let val = env.eval(args.get(1)?.as_ref())?;
            
            args
                .get(0)?
                .as_mut()
                .assign(&val);

            Ok(val)
        });

        self.add_bridge("gen-sym", |env, args| {
            let obj;
            
            if let Ok(val) = args.get(0) {
                obj = env.eval(val.as_ref())?;
            }
            else {
                obj = ().as_obj();
            }

            unsafe {
                let sym = env.gen_sym(obj);
                Ok(sym.as_obj())
            }
        });

        self.add_bridge("loop", |env, args| {
            let mut res = ().as_obj();
            let cond = args.get(0)?;

            while *env.eval(cond.as_ref())?.is_bool()? {
                res = args
                    .shift()
                    .progn(|obj| env.eval(obj.as_ref()))?;
            }
            
            Ok(res)
        });

        self.add_bridge("defun", |env, node| {
            let sym = node.get(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = node
                .get(1)?
                .as_ref()
                .is_node()?
                .clone();

            let body = node
                .skip(2)
                .cloned()
                .collect();

            let native = FnNative::new(name, params, body);

            sym.as_mut().assign_to(native);
            Ok(node.get(0)?.as_ref().clone())
        });

        self.add_bridge("lambda", |env, node| {
            let name = Env::unique_sym();

            let params = node
                .get(0)?
                .as_ref()
                .is_node()?
                .clone();

            let body = node
                .skip(1)
                .cloned()
                .collect();

            let native = FnNative::new(name, params, body);
            Ok(native.as_obj())
        });

        self.add_bridge("let", |env, args| {
            let fst = args
                .get(0)?
                .as_ref();
            
            let params = fst
                .is_node()?
                .iter()
                .step_by(2);

            let inputs = fst
                .is_node()?
                .iter()
                .skip(1)
                .step_by(2);

            args
                .shift()
                .progn_scoped(env, params, inputs)
        });

        self.add_bridge("do", |env, args| {
            args
                .progn(|obj| env.eval(obj.as_ref()))
        });

        self.add_bridge("defmacro", |env, node| {
            let sym = node.get(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = node
                .get(1)?
                .as_ref()
                .is_node()?
                .clone();

            let body = node
                .skip(2)
                .cloned()
                .collect();

            let macro_native = FnMacro::new(name, params, body);

            sym.as_mut().assign_to(macro_native);
            Ok(node.get(0)?.as_ref().clone())
        });

        self.add_bridge("macro-expand", |env, node| {
            let node = node.get(0)?.as_ref();
            let node = node.is_node()?;

            let fst = node.get(0)?;

            match env.eval(fst.as_ref())? {
                Obj::Macro(f)  => f.expand(env, node.iter_from(1)),   
                _ => Err(MisType)        
            }     
        });

        self.add_bridge("type-of", |env, args| {
            let res = env.eval(args.get(0)?.as_ref())?;
            Ok(res.type_string().as_obj())
        });

        self.add_bridge("quote", |_, args| {
            Ok(args.get(0)?.as_ref().clone())
        });

        self.add_bridge("eval", |env, args| {
            let fst = env.eval(args.get(0)?.as_ref())?;
            env.eval(&fst)
        });

        self.add_bridge("assert", |env, args| {
            let res = env
                .eval(args.get(0)?.as_ref())?
                .eq(&true.as_obj())?;

            if res {
                Ok(true.as_obj())
            } else {
                Err(RuntimeAssert)
            }
        });

        self.add_bridge("assert-eq", |env, args| {
            let res = env
                .eval(args.get(0)?.as_ref())?
                .eq(&env.eval(args.get(1)?.as_ref())?)?;

            if res {
                Ok(true.as_obj())
            } else {
                Err(RuntimeAssert)
            }
        });

        self.add_bridge("if", |env, args| {
            let cond = *env
                .eval(args.get(0)?.as_ref())?
                .is_bool()?;

            if cond {
                env.eval(args.get(1)?.as_ref())
            } 
            else {
                match args.get(2) {
                    Ok(or) => env.eval(or.as_ref()),
                    Err(_) => Ok(().as_obj())
                }
            }
        });

        self.add_bridge("when", |env, args| {
            let cond = *env
                .eval(args.get(0)?.as_ref())?
                .is_bool()?;

            if cond {
                args
                    .shift()
                    .progn(|obj| env.eval(obj.as_ref()))
            } 
            else {
                Ok(().as_obj())
            }
        });
        
        self.add_bridge("unless", |env, args| {
            let cond = *env
                .eval(args.get(0)?.as_ref())?
                .is_bool()?;

            if !cond {
                args
                    .shift()
                    .progn(|obj| env.eval(obj.as_ref()))
            } 
            else {
                Ok(().as_obj())
            }
        });
    }
}