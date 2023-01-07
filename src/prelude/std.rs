use crate::core::{
    env::Env,
    type_id::TypeId, 
    fun::{FnNative, FnMacro},
    err::ErrType::*, obj::Obj, obj::*, node::Node, rc_cell::RcCell
};

impl Env {
    pub fn std_lib(&mut self) {

        // constant Nil - nil
        self.add_primitive("Nil", ());

        // constant True - true
        self.add_primitive("True", true);

        // constant False - false
        self.add_primitive("False", false);
        
        // (set lhs rhs)
        self.add_bridge("set", |env, args| {
            let rhs = args
                .get(1)?
                .eval(env)?;

            args
                .get_cell(0)?
                .as_mut()
                .assign(&rhs);
            
            Ok(rhs)
        });

        // (gen-sym)
        self.add_bridge("gen-sym", |env, _| {            
            unsafe {
                let sym = env.gen_sym_runtime(Obj::Nil(()));
                Ok(sym.as_obj())
            }
        });

        // (loop cond ..body)
        self.add_bridge("loop", |env, args| {
            let mut ret = Obj::Nil(());

            while *args.get(0)?.eval(env)?.is_bool()? {
                ret = args
                    .shift()
                    .progn(|obj| env.eval(obj.as_ref()))?;
            }
            
            Ok(ret)
        });

        // (defun symbol params ..body)
        self.add_bridge("defun", |env, args| {
            let sym = args.get_cell(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = args
                .get(1)?
                .sym_value()?
                .is_node()?
                .clone();

            let body = args
                .skip(2)
                .cloned()
                .collect();

            let native = FnNative::new(name, params, body, false);
            sym.as_mut().assign_to(native);

            Ok(sym.as_ref().clone())
        });

        // (defun* name params ..body)
        self.add_bridge("defun*", |env, args| {
            let sym = args.get_cell(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = args
                .get(1)?
                .sym_value()?
                .is_node()?
                .clone();

            let body = args
                .skip(2)
                .cloned()
                .collect();

            let native = FnNative::new(name, params, body, true);
            sym.as_mut().assign_to(native);

            Ok(sym.as_ref().clone())
        });

        // (lambda params ..body)
        self.add_bridge("lambda", |_, args| {
            let params = args
                .get(0)?
                .sym_value()?
                .is_node()?
                .clone();
                
            let body = args
                .skip(1)
                .cloned()
                .collect();
            
            let name = Env::unique_sym();
            let native = FnNative::new(name, params, body, false);

            Ok(native.as_obj())
        });

        // (let params ..body)
        self.add_bridge("let", |env, args| {
            let fst = args
                .get(0)?
                .sym_value()?;
            
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

        // (do ..body)
        self.add_bridge("do", |env, args| {
            args.progn(|obj| env.eval(obj.as_ref()))
        });

        // (defmacro params ..body)
        self.add_bridge("defmacro", |env, args| {
            let sym = args.get_cell(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = args
                .get(1)?
                .sym_value()?
                .is_node()?
                .clone();

            let body = args
                .skip(2)
                .cloned()
                .collect();

            let native = FnMacro::new(name, params, body, false);
            sym.as_mut().assign_to(native);

            Ok(sym.as_ref().clone())
        });

        // (defmacro* params ..body)
        self.add_bridge("defmacro*", |env, args| {
            let sym = args.get_cell(0)?;

            let name = env
                .get_sym_id(sym.as_ref().is_symbol()?)
                .unwrap();

            let params = args
                .get(1)?
                .sym_value()?
                .is_node()?
                .clone();

            let body = args
                .skip(2)
                .cloned()
                .collect();

            let native = FnMacro::new(name, params, body, true);
            sym.as_mut().assign_to(native);

            Ok(sym.as_ref().clone())
        });

        // (macro-expand macro)
        self.add_bridge("macro-expand", |env, node| {
            let mac = node
                .get(0)?
                .sym_value()?
                .is_node()?;

            match mac.get(0)?.eval(env)? {
                Obj::Macro(f) => f.expand(env, mac.iter_from(1)),   
                _ => Err(MisType)        
            }     
        });

        // (type-of item)
        self.add_bridge("type-of", |env, args| {
            let item = args.get(0)?.eval(env)?;
            Ok(item.type_string().as_obj())
        });

        // (quote item)
        self.add_bridge("quote", |_, args| {
            Ok(args.get(0)?.clone())
        });

        // (eval item)
        self.add_bridge("eval", |env, args| {
            args.get(0)?
                .eval(env)?
                .eval(env)
        });

        // (assert cond)
        self.add_bridge("assert", |env, args| {
            let cond = args
                .get(0)?
                .eval(env)?
                .eq(&Obj::Bool(true))?;

            if cond {
                Ok(Obj::Bool(true))
            } else {
                Err(RuntimeAssert)
            }
        });

        // (assert-eq lhs rhs)
        self.add_bridge("assert-eq", |env, args| {
            let cond = args
                .get(0)?
                .eval(env)?
                .eq(&args.get(1)?.eval(env)?)?;

            if cond {
                Ok(Obj::Bool(true))
            } else {
                Err(RuntimeAssert)
            }
        });

        // (if cond then else)
        self.add_bridge("if", |env, args| {
            let cond = *args
                .get(0)?
                .eval(env)?
                .is_bool()?;

            if cond {
                args
                    .get(1)?
                    .eval(env)
            } 
            else {
                match args.get_cell(2) {
                    Ok(not) => not.as_ref().eval(env),
                    Err(_) => Ok(Obj::Nil(()))
                }
            }
        });

        // (when cond ..then)
        self.add_bridge("when", |env, args| {
            let cond = *args
                .get(0)?
                .eval(env)?
                .is_bool()?;

            if cond {
                args
                    .shift()
                    .progn(|obj| obj.as_ref().eval(env))
            } 
            else {
                Ok(Obj::Nil(()))
            }
        });
        
        // (unles cond ..then)
        self.add_bridge("unless", |env, args| {
            let cond = *args
                .get(0)?
                .eval(env)?
                .is_bool()?;

            if !cond {
                args
                    .shift()
                    .progn(|obj| obj.as_ref().eval(env))
            } 
            else {
                Ok(Obj::Nil(()))
            }
        });

        // (apply ..items)
        self.add_bridge("apply", |env, args| {
            let mut node = Node::from(vec![args.get_cell(0)?.clone()]);

            for item in args.skip(1) {
                let obj = item
                    .as_ref()
                    .sym_value()?;

                match &obj {
                    Obj::Lst(lst) => {
                        for elem in lst.iter() {
                            node.push(elem.clone());
                        }
                    }
                    _ => node.push(RcCell::from(obj.clone()))
                }
            }
            
            node.as_obj().eval(env)
        });
    }
}