use std::{cell::{RefCell, Ref, RefMut}, rc::Rc, ops::{DerefMut, Deref}};
use super::{type_id::TypeId, obj::Obj::{self, *}};

pub struct RcCell<T> {
    raw: Rc<RefCell<T>>
}

impl<T> Clone for RcCell<T> {
    fn clone(&self) -> Self {
        Self { 
            raw: Rc::clone(&self.raw)
        }
    }
}

impl Default for RcCell<Obj> {
    fn default() -> Self {
        Self::from(Nil())
    }
}

impl<T> From<T> for RcCell<T> {
    fn from(raw: T) -> Self {
        Self { 
            raw: Rc::new(RefCell::new(raw))
        }
    }
}

impl<T> From<Rc<RefCell<T>>> for RcCell<T> {
    fn from(raw: Rc<RefCell<T>>) -> Self {
        Self { 
            raw: raw
        }
    }
}

impl<T: PartialEq> PartialEq for RcCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> RcCell<T> {
    pub fn as_ref(&self) -> Ref<T> {
        self.raw.borrow()
    }

    pub fn as_mut(&self) -> RefMut<T> {
        self.raw.borrow_mut()
    }

    pub fn as_raw(&self) -> &Rc<RefCell<T>> {
        &self.raw
    }

    pub fn raw_cmp(&self, other: &Self) -> bool {
        self.as_raw().as_ptr() == other.as_raw().as_ptr()
    }
}