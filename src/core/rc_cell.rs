use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};
use super::{type_id::TypeId, object::Obj::{self, *}};


#[derive(Clone)]
pub struct RcCell<T: Clone> {
    raw: Rc<RefCell<T>>
}

impl TypeId for RcCell<Obj> {
    fn into_obj(self) -> Obj {
        todo!()
    }
}

impl Default for RcCell<Obj> {
    fn default() -> Self {
        Self::from(Nil())
    }
}

impl<T: Clone> From<T> for RcCell<T> {
    fn from(raw: T) -> Self {
        Self { 
            raw: Rc::new(RefCell::new(raw))
        }
    }
}

impl<T: Clone> From<Rc<RefCell<T>>> for RcCell<T> {
    fn from(raw: Rc<RefCell<T>>) -> Self {
        Self { 
            raw: raw
        }
    }
}

impl<T: Clone + PartialEq> PartialEq for RcCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T: Clone> RcCell<T> {
    pub fn as_ref(&self) -> Ref<T> {
        self.raw.borrow()
    }

    pub fn as_mut(&self) -> RefMut<T> {
        self.raw.borrow_mut()
    }

    pub fn as_raw(&self) -> &Rc<RefCell<T>> {
        &self.raw
    }
}