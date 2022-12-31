use super::{
    node::Node,
    rc_cell::RcCell,
    obj::Obj::{self, *}, 
    err::{Err, ErrType::*}, 
    fun::{FnNative, FnBridge, Callable, FnMacro}, 
    env::Env
};

/// A trait for designating `Jester-rs` types
pub trait TypeId: Clone {
    fn as_obj(self) -> Obj;
    fn type_str() -> &'static str;
    fn as_string(&self, env: &Env) -> String;
}

/// Marks `Jester-rs` type as primitive
pub trait Primitive: Default + TypeId {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for f64 {}
impl Primitive for bool {}
impl Primitive for () {}

/// Marks `Jester-rs` type as numeric
pub trait Numeric: Primitive {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for f64 {}

impl TypeId for f64 {
    fn as_obj(self) -> Obj {
        F64(self)
    }

    fn type_str() -> &'static str {
        "float"
    }

    fn as_string(&self, _: &Env) -> String {
        self.to_string()
    }
}

impl TypeId for i32 {
    fn as_obj(self) -> Obj {
        I32(self)
    }

    fn type_str() -> &'static str {
        "i32"
    }

    fn as_string(&self, _: &Env) -> String {
        self.to_string()
    }
}

impl TypeId for i64 {
    fn as_obj(self) -> Obj {
        I64(self)
    }

    fn type_str() -> &'static str {
        "i64"
    }

    fn as_string(&self, _: &Env) -> String {
        self.to_string()
    }
}

impl TypeId for i128 {
    fn as_obj(self) -> Obj {
        I128(self)
    }

    fn type_str() -> &'static str {
        "i128"
    }

    fn as_string(&self, _: &Env) -> String {
        self.to_string()
    }
}

impl TypeId for bool {
    fn as_obj(self) -> Obj {
        Bool(self)
    }
    
    fn type_str() -> &'static str {
        "bool"
    }

    fn as_string(&self, _: &Env) -> String {
        self.to_string()
    }
}

impl TypeId for String {
    fn as_obj(self) -> Obj {
        Str(self)
    }

    fn type_str() -> &'static str {
        "string"
    }

    fn as_string(&self, _: &Env) -> String {
        format!("\"{}\"", self)
    }
}

impl TypeId for RcCell<Obj> {
    fn as_obj(self) -> Obj {
        Sym(self)
    }

    fn type_str() -> &'static str {
        "symbol"
    }

    fn as_string(&self, env: &Env) -> String {
        if let Ok(list) = self.clone_inner().is_node() {
            return list.as_string(env);
        }

        env
            .get_sym_id(self)
            .unwrap()
            .clone()
    }
}

impl TypeId for FnNative {
    fn as_obj(self) -> Obj {
        Native(self)
    }

    fn type_str() -> &'static str {
        "native()"
    }

    fn as_string(&self, env: &Env) -> String {
        let params = self.params().as_string(env);
        format!("{}{}", self.name(), params)
    }
}

impl TypeId for FnMacro {
    fn as_obj(self) -> Obj {
        Macro(self)
    }

    fn type_str() -> &'static str {
        "macro()"
    }

    fn as_string(&self, env: &Env) -> String {
        let params = self.params().as_string(env);
        format!("{}{}", self.name(), params)
    }
}

impl TypeId for FnBridge {
    fn as_obj(self) -> Obj {
        Bridge(self)
    }

    fn type_str() -> &'static str {
        "bridge()"
    }

    fn as_string(&self, _: &Env) -> String {
        format!("{}()", self.name())
    }
}

impl TypeId for Node {
    fn as_obj(self) -> Obj {
        Lst(self)
    }

    fn type_str() -> &'static str {
        "node"
    }

    fn as_string(&self, env: &Env) -> String {
        self
            .iter()
            .map(|obj| obj.as_ref().to_string(env))
            .reduce(|acc, next| format!("{} {}", acc, next))
            .map_or(
                "()".to_string(), 
                |list| format!("({})", list))
    }
}

impl TypeId for () {
    fn as_obj(self) -> Obj {
        Nil(())
    }

    fn type_str() -> &'static str {
        "nil"
    }

    fn as_string(&self, _: &Env) -> String {
        Self::type_str().to_string()
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
        match TypeId::as_obj(T::default()) {           
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

    /// Returns `Ok(Num)` if `self` is of type
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

    /// Returns `Ok(Int)` if `self` is of type
    /// ```
    /// i32, i64, i128
    /// ```
    pub fn is_int(&self) -> Err<i128> {
        match self {
            I32(x)  => Ok(*x as i128),
            I64(x)  => Ok(*x as i128),
            I128(x) => Ok(*x as i128),
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
    pub fn is_node(&self) -> Err<&Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut Node)` if `self` is of type
    /// ```
    /// List
    /// ```
    pub fn is_node_mut(&mut self) -> Err<&mut Node> {
        match self {
            Lst(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&RcCell<Obj>)` if `self` is of type
    /// ```
    /// Symbol
    /// ```
    pub fn is_symbol(&self) -> Err<&RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }

    /// Returns `Ok(&mut RcCell<Obj>)` if `self` is of type
    /// ```
    /// Symbol
    /// ```
    pub fn is_symbol_mut(&mut self) -> Err<&mut RcCell<Obj>> {
        match self {
            Sym(x) => Ok(x),
            _ => Err(MisType)
        }
    }
}