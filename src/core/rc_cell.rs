use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use super::{obj::Obj, err::Err};

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
    pub fn as_ref(&self) -> &T {
        unsafe {
            self.raw
                .as_ptr()
                .as_ref()
                .unwrap()
        }    
    }

    pub fn as_mut(&self) -> &mut T {
        unsafe {
            self.raw
                .as_ptr()
                .as_mut()
                .unwrap()
        }
    }

    pub fn as_raw(&self) -> &Rc<RefCell<T>> {
        &self.raw
    }

    pub fn clone_inner(&self) -> T 
    where
        T: Clone
    {
        self.as_ref().clone()
    }

    pub fn raw_eq(&self, other: &Self) -> bool {
        self.as_raw().as_ptr() == other.as_raw().as_ptr()
    }
}

impl RcCell<Obj> {

    pub fn map_inner<F>(&self, mut map: F) -> Err<Obj>
    where
        F: FnMut(&mut Obj) -> Err<Obj>
    {
        let mut sym = self.as_mut();
        let sym = sym.is_symbol_mut()?;

        map(sym.as_mut())
    }
}