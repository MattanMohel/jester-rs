use crate::core::{
    env::Env,
    err::Err, 
    obj::Obj, rc_cell::RcCell, type_id::TypeId,
};

impl Env {
    pub fn list_lib(&mut self) {

        self.add_bridge("len", |env, args| {
            let len = env
                .eval(args.get(0)?.as_ref())?
                .is_node()?
                .len();

            Ok((len as i64).as_obj())
        });

        self.add_bridge("nth", |env, args| {
            let idx = args
                .get(0)?
                .as_ref()
                .is_int()?;

            let nth = args
                .get(1)?
                .as_ref();
            
            let res = env.eval(nth.is_node()?.get(idx as usize)?.as_ref())?;

            Ok(res)
        });

        self.add_bridge("replace", |env, args| {
            args
                .get(2)?
                .map_inner(|obj| {
                    let index = env
                        .eval(args.get(0)?.as_ref())?
                        .is_int()? as usize;
                        
                    let elem = env.eval(args.get(1)?.as_ref())?;

                    let list = obj.is_node()?;

                    list
                        .get(index)?
                        .as_mut()
                        .assign(&elem);

                    Ok(list.get(index)?.clone_inner())
                })
        });

        self.add_bridge("append", |env, args| {     
            args
                .get(1)?
                .map_inner(|mut list| {
                    let elem = env.eval(args.get(0)?.as_ref())?;

                    list
                        .is_node_mut()?
                        .push(RcCell::from(elem.clone()));

                    Ok(elem)
                })
        });

        self.add_bridge("prepend", |env, args| {
            args
                .get(1)?
                .map_inner(|mut list| {
                    let elem = env.eval(args.get(0)?.as_ref())?;

                    list
                        .is_node_mut()?
                        .insert(0, RcCell::from(elem.clone()));

                    Ok(elem)
                })
        });

        self.add_bridge("insert", |env, args| {
            args
                .get(2)?
                .map_inner(|mut list| {
                    let idx = env
                        .eval(args.get(0)?.as_ref())?
                        .is_int()?;
    
                    let elem = env.eval(args.get(1)?.as_ref())?;
        
                    list
                        .is_node_mut()?
                        .insert(idx as usize, RcCell::from(elem.clone()));
                    
                    Ok(elem)
                })
        });

        self.add_bridge("remove", |env, args| {
            args
                .get(1)?
                .map_inner(|mut list| {
                    let idx = env
                        .eval(args.get(0)?.as_ref())?
                        .is_int()?;
        
                    list
                        .is_node_mut()?
                        .remove(idx as usize)
                })
        });
    }
}