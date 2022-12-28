use super::{
    node::Node,
    rc_cell::RcCell,
    obj::Obj::{self, *}, 
    err::{Err, ErrType::*}, fun::FnNative
};

/// A trait for designating `Jester-rs` types
pub trait TypeId: Default + Clone {
    fn into_obj(self) -> Obj;
    fn type_str() -> &'static str;
}

/// Marks `Jester-rs` type as primitive
pub trait Primitive: TypeId {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for f64 {}
impl Primitive for bool {}

/// Marks `Jester-rs` type as numeric
pub trait Numeric: Primitive {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f64 {}

impl TypeId for f64 {
    fn into_obj(self) -> Obj {
        F64(self)
    }

    fn type_str() -> &'static str {
        "float"
    }
}

impl TypeId for i32 {
    fn into_obj(self) -> Obj {
        I32(self)
    }

    fn type_str() -> &'static str {
        "i32"
    }
}

impl TypeId for i64 {
    fn into_obj(self) -> Obj {
        I64(self)
    }

    fn type_str() -> &'static str {
        "i64"
    }
}

impl TypeId for i128 {
    fn into_obj(self) -> Obj {
        I128(self)
    }

    fn type_str() -> &'static str {
        "i128"
    }
}

impl TypeId for bool {
    fn into_obj(self) -> Obj {
        Bool(self)
    }
    
    fn type_str() -> &'static str {
        "bool"
    }
}

impl TypeId for String {
    fn into_obj(self) -> Obj {
        Str(self)
    }

    fn type_str() -> &'static str {
        "string"
    }
}

impl TypeId for RcCell<Obj> {
    fn into_obj(self) -> Obj {
        Obj::Sym(self)
    }

    fn type_str() -> &'static str {
        "symbol"
    }
}

impl TypeId for Node {
    fn into_obj(self) -> Obj {
        Obj::Lst(self)
    }

    fn type_str() -> &'static str {
        "node"
    }
}

impl TypeId for () {
    fn into_obj(self) -> Obj {
        Nil()
    }

    fn type_str() -> &'static str {
        "nil"
    }
}

impl Obj {
    /// Coerces `Obj` into  `T: Primitive`
    /// 
    /// ## Note
    /// `self` must be `Primitive`
    /// ```
    /// Primitive: f64, i32, i64, i128, bool
    /// ```
    pub unsafe fn cast_as<T: Primitive>(&self) -> Err<T> {
        match TypeId::into_obj(T::default()) {           
            F64(x)  => Ok(std::mem::transmute_copy::<f64, T>(&x)),
            I32(x)  => Ok(std::mem::transmute_copy::<i32, T>(&x)),
            I64(x)  => Ok(std::mem::transmute_copy::<i64, T>(&x)),
            I128(x) => Ok(std::mem::transmute_copy::<i128, T>(&x)),
            Bool(x) => Ok(std::mem::transmute_copy::<bool, T>(&x)),
            _ => Err(ErrCast)
        }
    }

    /// Coerces `Obj` into  `i32`
    /// 
    /// ## Note
    /// `self` must be `Primitive`
    /// ```
    /// Primitive: f64, i32, i64, i128, bool
    /// ```
    pub fn as_i32(&self) -> Err<i32> {
        match *self {
            F64(x)  => Ok(x as i32),
            I32(x)  => Ok(x as i32),
            I64(x)  => Ok(x as i32),
            I128(x) => Ok(x as i32),
            Bool(x) => Ok(x as i32),
            _ => Err(ErrCast)
        }
    }

    /// Coerces `Obj` into  `i64`
    /// 
    /// ## Note
    /// `self` must be `Primitive`
    /// ```
    /// Primitive: f64, i32, i64, i128, bool
    /// ```
    pub fn as_i64(&self) -> Err<i64> {
        match *self {
            F64(x)  => Ok(x as i64),
            I32(x)  => Ok(x as i64),
            I64(x)  => Ok(x as i64),
            I128(x) => Ok(x as i64),
            Bool(x) => Ok(x as i64),
            _ => Err(ErrCast)
        }
    }

    /// Coerces `Obj` into  `i128`
    /// 
    /// ## Note
    /// `self` must be `Primitive`
    /// ```
    /// Primitive: f64, i32, i64, i128, bool
    /// ```
    pub fn as_i128(&self) -> Err<i128> {
        match *self {
            F64(x)  => Ok(x as i128),
            I32(x)  => Ok(x as i128),
            I64(x)  => Ok(x as i128),
            I128(x) => Ok(x as i128),
            Bool(x) => Ok(x as i128),
            _ => Err(ErrCast)
        }
    }

    /// Coerces `Obj` into  `f64`
    /// 
    /// ## Note
    /// `self` must be `Numeric`
    /// ```
    /// Numeric: f64, i32, i64, i128
    /// ```
    pub fn as_f64(&self) -> Err<f64> {
        match *self {
            F64(x)  => Ok(x as f64),
            I32(x)  => Ok(x as f64),
            I64(x)  => Ok(x as f64),
            I128(x) => Ok(x as f64),
            _ => Err(ErrCast)
        }
    }

    /// Returns `Ok(num)` if `self` is of type
    /// ```
    /// f64, i32, i64, i128
    /// ```
    pub fn is_num(&self) -> Err<f64> {
        match self {
            I32(x)  => Ok(*x as f64),
            I64(x)  => Ok(*x as f64),
            I128(x) => Ok(*x as f64),
            F64(x)  => Ok(*x as f64),
            _ => Err(MisType)
        }
    } 

    /// Returns `Ok(int)` if `self` is of type
    /// ```
    /// i32, i64, i128
    /// ```
    pub fn is_int(&self) -> Err<u64> {
        match self {
            I32(x) => Ok(*x as u64),
            I64(x) => Ok(*x as u64),
            _ => Err(MisType)
        }
    } 

    /// Returns `Ok(&i32)` if `self` is of type
    /// ```
    /// i32
    /// ```
    pub fn is_i32(&self) -> Err<&i32> {
        match self {
            I32(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut i32)` if `self` is of type
    /// ```
    /// i32
    /// ```
    pub fn is_i32_mut(&mut self) -> Err<&mut i32> {
        match self {
            I32(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&i64)` if `self` is of type
    /// ```
    /// i64
    /// ```
    pub fn is_i64(&self) -> Err<&i64> {
        match self {
            I64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut i64)` if `self` is of type
    /// ```
    /// i64
    /// ```
    pub fn is_i64_mut(&mut self) -> Err<&mut i64> {
        match self {
            I64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&i128)` if `self` is of type
    /// ```
    /// i128
    /// ```
    pub fn is_i128(&self) -> Err<&i128> {
        match self {
            I128(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut i128)` if `self` is of type
    /// ```
    /// i128
    /// ```
    pub fn is_i128_mut(&mut self) -> Err<&mut i128> {
        match self {
            I128(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&f64)` if `self` is of type
    /// ```
    /// f64
    /// ```
    pub fn is_f64(&self) -> Err<&f64> {
        match self {
            F64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut f64)` if `self` is of type
    /// ```
    /// f64
    /// ```
    pub fn is_f64_mut(&mut self) -> Err<&mut f64> {
        match self {
            F64(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&bool)` if `self` is of type
    /// ```
    /// bool
    /// ```
    pub fn is_bool(&self) -> Err<&bool> {
        match self {
            Bool(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut bool)` if `self` is of type
    /// ```
    /// bool
    /// ```
    pub fn is_bool_mut(&mut self) -> Err<&mut bool> {
        match self {
            Bool(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&String)` if `self` is of type
    /// ```
    /// String
    /// ```
    pub fn is_string(&self) -> Err<&String> {
        match self {
            Str(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut String)` if `self` is of type
    /// ```
    /// String
    /// ```
    pub fn is_string_mut(&mut self) -> Err<&mut String> {
        match self {
            Str(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&Node)` if `self` is of type
    /// ```
    /// List
    /// ```
    pub fn is_list(&self) -> Err<&Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut Node)` if `self` is of type
    /// ```
    /// List
    /// ```
    pub fn is_list_mut(&mut self) -> Err<&mut Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&RcCell<Obj>)` if `self` is of type
    /// ```
    /// Symbol
    /// ```
    pub fn is_quote(&self) -> Err<&RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut RcCell<Obj>)` if `self` is of type
    /// ```
    /// Symbol
    /// ```
    pub fn is_quote_mut(&mut self) -> Err<&mut RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }
}