use crate::core::{
    env::Env,
    rc_cell::RcCell, 
    type_id::TypeId
};

impl Env {
    pub fn list_lib(&mut self) {

        self.add_bridge("len", |env, args| {
            let len = env
                .eval(args.get(0)?)?
                .is_node()?
                .len();

            Ok((len as i64).as_obj())
        });

        self.add_bridge("nth", |env, args| {
            let idx = env
                .eval(args.get(0)?)?
                .is_int()?;

            let nth = env
                .eval(args.get(1)?)?;               

            let res = nth.is_node()?.get_cell(idx as usize)?.clone_inner();
            
            Ok(res)
        });

        self.add_bridge("replace", |env, args| {
            args
                .get_cell(2)?
                .map_inner(|obj| {
                    let index = env
                        .eval(args.get(0)?)?
                        .is_int()? as usize;
                        
                    let elem = env.eval(args.get(1)?)?;

                    let list = obj.is_node()?;

                    *list.get_cell(index)?.as_mut() = elem;

                    Ok(list.get_cell(index)?.clone_inner())
                })
        });

        self.add_bridge("append", |env, args| {     
            args
                .get_cell(1)?
                .map_inner(|list| {
                    let elem = env.eval(args.get(0)?)?;

                    list
                        .is_node_mut()?
                        .push(RcCell::from(elem.clone()));

                    Ok(elem)
                })
        });

        self.add_bridge("prepend", |env, args| {
            args
                .get_cell(1)?
                .map_inner(|list| {
                    let elem = env.eval(args.get(0)?)?;

                    list
                        .is_node_mut()?
                        .insert(0, RcCell::from(elem.clone()));

                    Ok(elem)
                })
        });

        self.add_bridge("insert", |env, args| {
            args
                .get_cell(2)?
                .map_inner(|list| {
                    let idx = env
                        .eval(args.get(0)?)?
                        .is_int()?;
    
                    let elem = env.eval(args.get(1)?)?;
        
                    list
                        .is_node_mut()?
                        .insert(idx as usize, RcCell::from(elem.clone()));
                    
                    Ok(elem)
                })
        });

        self.add_bridge("remove", |env, args| {
            args
                .get_cell(1)?
                .map_inner(|list| {
                    let idx = env
                        .eval(args.get(0)?)?
                        .is_int()?;
        
                    list
                        .is_node_mut()?
                        .remove(idx as usize)
                })
        });
    }
}