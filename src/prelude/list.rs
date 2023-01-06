use crate::core::{
    env::Env,
    rc_cell::RcCell, 
    type_id::TypeId
};

impl Env {
    pub fn list_lib(&mut self) {

        // (len list)
        self.add_bridge("len", |_, args| {
            let len = args
                .get(0)?
                .sym_value()?
                .is_node()?
                .len();

            Ok((len as i64).as_obj())
        });

        // (nth list)
        self.add_bridge("nth", |env, args| {
            let idx = args
                .get(0)?
                .eval(env)?
                .is_int()?;
            
            Ok(args
                .get(1)?
                .sym_val_mut()?
                .is_node_mut()?
                .get_cell(idx as usize)?
                .clone_inner())
        });

        // (replace index value list)
        self.add_bridge("replace", |env, args| {
            let [index, value] = env.eval_args([0, 1], args)?;

            let list = args
                .get(2)?
                .sym_val_mut()?
                .is_node()?;

            let elem = list.get_mut(index.is_int()? as usize)?;
            let copy = elem.clone();
            *elem = value;
                        
            Ok(copy)
        });

        // (append value list)
        self.add_bridge("append", |env, args| {     
            let value = args
                .get(0)?
                .eval(&env)?;

            let list = args
                .get(1)?
                .sym_val_mut()?
                .is_node_mut()?;

            list.push(RcCell::from(value.clone()));

            Ok(value)
        });

        // (prepend item list)
        self.add_bridge("prepend", |env, args| {
            let item = args
                .get(0)?
                .eval(env)?
                .clone();

            let list = args
                .get(1)?
                .sym_val_mut()?
                .is_node_mut()?;

            list.insert(0, RcCell::from(item.clone()))?;        
            Ok(item)
        });

        // (insert index item list)
        self.add_bridge("insert", |env, args| {
            let [index, item] = env.eval_args([0, 1], args)?;

            let list = args
                .get(2)?
                .sym_val_mut()?
                .is_node_mut()?;

            list.insert(index.is_int()? as usize, RcCell::from(item.clone()))?;  
            Ok(item)
        });

        // (remove index list)
        self.add_bridge("remove", |env, args| {
            let index = args
                .get(0)?
                .eval(env)?
                .is_int()? as usize;

            let list = args
                .get(1)?
                .sym_val_mut()?
                .is_node_mut()?;

            let rem = list.remove(index)?;        
            Ok(rem)
        });
    }
}