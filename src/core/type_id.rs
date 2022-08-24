use super::object::Obj::{self, *};

pub trait TypeId: Default + Clone {
    fn into_obj(self) -> Obj;
}

impl TypeId for i32 {
    fn into_obj(self) -> Obj {
        I32(self)
    }
}
impl TypeId for i64 {
    fn into_obj(self) -> Obj {
        I64(self)
    }
}
impl TypeId for i128 {
    fn into_obj(self) -> Obj {
        I128(self)
    }
}
impl TypeId for f64 {
    fn into_obj(self) -> Obj {
        F64(self)
    }
}

impl TypeId for bool {
    fn into_obj(self) -> Obj {
        Bool(self)
    }
}

impl TypeId for String {
    fn into_obj(self) -> Obj {
        Str(self)
    }
}

impl TypeId for () {
    fn into_obj(self) -> Obj {
        Nil()
    }
}