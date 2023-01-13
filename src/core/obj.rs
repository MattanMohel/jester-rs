use super::{
    rc_cell::RcCell, 
    type_id::TypeId,
    node::Node, 
    env::Env,
    err::{Err, ErrType::*},
    fun::{FnNative, Bridge, FnBridge, FnMacro}
};

/// `Jester-rs` representation of data
#[derive(Clone)]
pub enum Obj {
    /// `symbol`
    Sym(RcCell<Obj>),
    /// `list`
    Lst(Node),
    /// `float: Primtive + Numeric`
    F64(f64),
    /// `i32: Primtive + Numeric`
    I32(i32),
    /// `i64: Primtive + Numeric`
    I64(i64),
    /// `i128: Primtive + Numeric`
    I128(i128),
    /// `bool: Primtive`
    Bool(bool),
    /// `string`
    Str(String),
    /// `native-fn`
    Native(FnNative),
    /// `bridge-fn`
    Bridge(FnBridge),
    /// `macro-fn`
    Macro(FnMacro),
    /// `nil`
    Nil(())
}

use Obj::*;

impl Obj {

    /// Evaluates `self`
    pub fn eval(&self, env: &Env) -> Err<Obj> {
        env.eval(self)
    }
    
    /// Assigns value to clone of `other`
    /// 
    /// ## Note
    /// Panics if not `Sym`
    pub fn assign(&mut self, other: &Self) {
        match self {
            Sym(obj) => *obj.as_mut() = other.clone(),
            _ => panic!("can't assign to non-symbol!")
        }
    }

    /// Assigns value to clone of `other` as an `Obj`
    ///     
    /// ## Note
    /// Panics if not `Sym`
    pub fn assign_to<T: TypeId>(&mut self, other: T) {
        match self {
            Sym(obj) => *obj.as_mut() = other.as_obj(),
            _ => panic!("can't assign to non-symbol!")
        }
    }

    /// Creates a new `Obj::T`
    pub fn new_value<T: TypeId>(val: T) -> Self {
        val.as_obj()
    }

    /// Creates a new `Obj::FnBridge(bridge)`
    pub fn new_bridge(sym: String, bridge: Bridge) -> Self {
        Bridge(FnBridge::new(sym, bridge))
    }

    /// Returns value for display
    pub fn display(&self, env: &Env) -> String {
        match self {
            Str(x) => format!("\"{}\"", x.as_string(env)),
            _ => self.as_string(env)
        }
    }

    /// Parses an `Obj` literal from a String, returning
    /// `None` if the `str` is not a `Jester-rs` literal 
    pub fn parse_literal(str: &String) -> Option<Self> {
        if let Ok(num) = Obj::sym_to_num(str) {
            Some(num)
        }
        else if let Some(str) = Obj::sym_to_str(str) {
            Some(str)
        }
        else {
            None
        }
    }

    pub fn sym_value(&self) -> Err<&Self> {
        match self {
            Sym(sym) => Ok(sym.as_ref()),
            _ => Err(MisType)
        }
    }

    pub fn sym_val_mut(&self) -> Err<&mut Self> {
        match self {
            Sym(sym) => Ok(sym.as_mut()),
            _ => Err(MisType)
        }
    }

    pub fn val(&self) -> &Self {
        match self {
            Sym(sym) => sym.as_ref(),
            _ => self
        }
    }

    pub fn val_mut(&mut self) -> &mut Self {
        match self {
            Sym(sym) => sym.as_mut(),
            _ => self
        }
    }

    /// Returns the value of object as String
    pub fn as_string(&self, env: &Env) -> String {
        match self {
            Sym(x)     => x.as_string(env),
            Lst(x)     => x.as_string(env),
            I32(x)     => x.as_string(env),
            I64(x)     => x.as_string(env),
            I128(x)    => x.as_string(env),
            F64(x)     => x.as_string(env),
            Bool(x)    => x.as_string(env),
            Str(x)     => x.as_string(env),
            Native(x)  => x.as_string(env),
            Bridge(x)  => x.as_string(env),
            Macro(x)   => x.as_string(env),
            Nil(x)     => x.as_string(env)
        }
    }

    /// Returns the type of object as String
    pub fn type_string(&self) -> String {
        match self {
            Sym(_)    => RcCell::<Self>::type_str(),
            Lst(_)    => Node::type_str(),
            I32(_)    => i32::type_str(),
            I64(_)    => i64::type_str(),
            I128(_)   => i128::type_str(),
            F64(_)    => f64::type_str(),
            Bool(_)   => bool::type_str(),
            Str(_)    => String::type_str(),
            Native(_) => FnNative::type_str(),
            Bridge(_) => FnBridge::type_str(),
            Macro(_)  => FnMacro::type_str(),
            Nil(_)    => <()>::type_str()
        }
        .to_string()
    }

    /// Tries to convert `String` into `Obj::Str`
    /// 
    /// ## Example
    /// ```
    /// Str  | "123" 
    /// Str  | "abc" 
    /// None |  123
    /// None |  abc
    /// ```
    pub fn sym_to_str(src: &String) -> Option<Self> {
        // asserts if src begins and ends with ""
        if &src[0..1] != "\"" || &src[src.len() - 1..] != "\"" {
            return None
        }
    
        // collects the string content 
        let col = src
            .chars()
            .skip(1)
            .take(src.len() - 2)
            .collect::<String>();
    
        Some(Str(col))
    }
       
    /// Tries to convert `String` into `Obj::Num`
    /// 
    /// ## Typing
    /// The num-type (`i32|i64|i128|f64`) is chosen
    /// based on the size of the parsed number
    /// 
    /// ## Delimiter
    /// Number can be separated by `_`
    /// ```
    /// i.e. 1_0__0_0__0.
    /// ```
    /// 
    /// ## Binary
    /// Numbers can be interpreted as binary
    /// ```
    /// i.e. #b0001_1111 == 31
    /// ```
    /// 
    /// ## Hexadecimal
    /// Numbers can be interpreted as hexadecimal
    /// ```
    /// i.e. #h6_68A0 == 420_000
    /// ```
    pub fn sym_to_num(str: &String) -> Err<Self> {
        // remove all '_'
        let str: String = str
            .chars()
            .filter(|ch| *ch != '_')
            .collect();

        if str.is_empty() {
            return Err(MisForm)
        }

        // index of first digit
        let mut fst_dig = None;
        // index of number end
        let mut num_end = 0;
        // index of decimal point
        let mut decimal_loc = None;
        // index of the sign
        let mut sign_loc = None;
        // sign multiplier -/+
        let mut sign = 1;

        let base;
        if &str[0..1] == "#" {
            match &str[1..2] {
                "b" => base = 2,
                "h" => base = 16,
                _ => return Err(MisForm)               
            }
        }
        else {
            base = 10;
        }

        for (i, ch) in str.chars().enumerate() {
            if fst_dig.is_none() && (ch == '.' || ch.is_digit(base)) {
                fst_dig = Some(i);
            }      
            
            if ch.is_digit(base) || ch == '.' || ch == '+' || ch == '-' { 
                match ch {
                    '.' => {
                        if decimal_loc.is_some() || base != 10 {
                            return Err(MisForm)
                        }

                        decimal_loc = Some(i);
                    }
                    '+' | '-' => {
                        if ch == '-' {
                            sign = -1;
                        }

                        if fst_dig.is_some() || sign_loc.is_some() || base != 10 {
                            return Err(MisForm)
                        }

                        sign_loc = Some(i);
                    }
                    _ => ()
                }

                num_end = i;
            }
        }

        // assert a first digit was found
        if fst_dig.is_none() || (num_end + 1) as usize != str.len() {
            return Err(MisForm)
        }
        
        let fst_dig = fst_dig.unwrap();

        // filter and collect into just digits
        let digits: Vec<i128> = str
            .chars()
            .filter(|ch| ch.is_digit(base))
            .map(|dig| dig.to_digit(base).unwrap() as i128)
            .collect();

        // sum digits^base(x) 
        let num = digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, dig)| acc + dig * (base as i128).pow(i as u32));


        if let Some(loc) = decimal_loc {
            let pow = (base as f64).powi((digits.len() - loc + fst_dig) as i32);
            let float = (sign * num) as f64 / pow;
            
            return Ok(F64(float));
        }

        if let Ok(int) = i32::try_from(sign * num) {
            Ok(I32(int))
        }
        else if let Ok(int) = i64::try_from(sign * num) {
            Ok(I64(int))
        }
        else if let Ok(int) = i128::try_from(sign * num) {
            Ok(I128(int))
        }
        else {
            Err(Overflow)
        }
    }
}