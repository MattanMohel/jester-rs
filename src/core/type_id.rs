use super::{
    node::Node,
    rc_cell::RcCell,
    obj::Obj::{self, *}, 
    err::{Err, ErrType::*}
};

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


impl Obj {
    /// coerces object into type T
    pub unsafe fn cast_as<T: TypeId>(&self) -> Err<T> {
        match TypeId::into_obj(T::default()) {           
            I32(_) => Ok(std::mem::transmute_copy::<i32, T>(&self.as_i32()?)),
            I64(_) => Ok(std::mem::transmute_copy::<i64, T>(&self.as_i64()?)),
            F64(_) => Ok(std::mem::transmute_copy::<f64, T>(&self.as_f64()?)),
            Str(_) => Ok(std::mem::transmute_copy::<String, T>(self.is_str()?)),
            Bool(_) => Ok(std::mem::transmute_copy::<bool, T>(self.is_bool()?)),
            _ => Err(MisType)
        }
    }

    pub fn is_num(&self) -> Err<f64> {
        match self {
            I32(x) => Ok(*x as f64),
            I64(x) => Ok(*x as f64),
            I128(x) => Ok(*x as f64),
            F64(x) => Ok(*x as f64),
            _ => Err(MisType)
        }
    } 

    pub fn is_int(&self) -> Err<u64> {
        match self {
            I32(x) => Ok(*x as u64),
            I64(x) => Ok(*x as u64),
            _ => Err(MisType)
        }
    } 

    pub fn is_i32(&self) -> Err<&i32> {
        match self {
            I32(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_i32_mut(&mut self) -> Err<&mut i32> {
        match self {
            I32(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_i64(&self) -> Err<&i64> {
        match self {
            I64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_i64_mut(&mut self) -> Err<&mut i64> {
        match self {
            I64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_i128(&self) -> Err<&i128> {
        match self {
            I128(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_i128_mut(&mut self) -> Err<&mut i128> {
        match self {
            I128(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_f64(&self) -> Err<&f64> {
        match self {
            F64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_f64_mut(&mut self) -> Err<&mut f64> {
        match self {
            F64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_bool(&self) -> Err<&bool> {
        match self {
            Bool(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_bool_mut(&mut self) -> Err<&mut bool> {
        match self {
            Bool(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_str(&self) -> Err<&String> {
        match self {
            Str(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_str_mut(&mut self) -> Err<&mut String> {
        match self {
            Str(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_list(&self) -> Err<&Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_list_mut(&mut self) -> Err<&mut Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_quote(&self) -> Err<&RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn is_quote_mut(&mut self) -> Err<&mut RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    pub fn as_i32(&self) -> Err<i32> {
        match *self {
            I32(x) => Ok(x as i32),
            I64(x) => Ok(x as i32),
            I128(x) => Ok(x as i32),
            F64(x) => Ok(x as i32),
            _ => Err(ErrCast),
        }
    }

    pub fn as_i64(&self) -> Err<i64> {
        match *self {
            I32(x) => Ok(x as i64),
            I64(x) => Ok(x as i64),
            I128(x) => Ok(x as i64),
            F64(x) => Ok(x as i64),
            _ => Err(ErrCast),
        }
    }

    pub fn as_i128(&self) -> Err<i128> {
        match *self {
            I32(x) => Ok(x as i128),
            I64(x) => Ok(x as i128),
            I128(x) => Ok(x as i128),
            F64(x) => Ok(x as i128),
            _ => Err(ErrCast),
        }
    }

    pub fn as_f64(&self) -> Err<f64> {
        match *self {
            I32(x) => Ok(x as f64),
            I64(x) => Ok(x as f64),
            I128(x) => Ok(x as f64),
            F64(x) => Ok(x as f64),
            _ => Err(ErrCast),
        }
    }
}