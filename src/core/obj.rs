use std::ops::Deref;

use super::{
    rc_cell::RcCell, 
    type_id::TypeId,
    node::Node, 
    env::Env,
    err::{Err, ErrType::*},
    fun::{FnNative, Bridge, FnBridge}
};



use Obj::*;

impl From<&String> for Obj {
    fn from(str: &String) -> Self {
        if let Ok(num) = Obj::sym_to_num(str) {
            num
        }
        else if let Some(str) = Obj::sym_to_str(str) {
            str
        }
        else {
            Obj::Nil()
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Obj {
    Sym(RcCell<Obj>),
    Lst(Node),
    
    I32(i32),
    I64(i64),
    I128(i128),
    F64(f64),

    Bool(bool),
    Str(String),

    FnNative(FnNative),
    FnBridge(FnBridge),

    Nil(),
}


impl Obj {
    /// Sets object value to clone of `other<Obj>`
    pub fn set(&mut self, other: &Obj) -> Err {
        match self {
            Sym(raw) => *raw.as_mut() = other.clone(),
            _ => return Err(MisType)
        }
        Ok(())
    }

    /// Sets object value to clone of `other<T>`
    pub fn set_to<T: TypeId>(&mut self, other: T) -> Err {
        match self {
            Sym(raw) => *raw.as_mut() = other.into_obj(),
            _ => return Err(MisType)
        }
        Ok(())    }

    /// Creates a new `Obj::FnBridge(exec)`
    pub fn new_bridge(exec: Bridge) -> Obj {
        Obj::FnBridge(FnBridge::from(exec))
    }

    /// Creates a new `Obj::T`
    pub fn new_value<T: TypeId>(val: T) -> Obj {
        val.into_obj()
    }

    /// Returns the value of object as String
    pub fn to_string(&self, env: &Env) -> String {
        match self {
            Sym(sym)    => env.cell_sym(sym).unwrap(),
            Lst(lst)    => lst.to_string(env),
            I32(num)    => num.to_string(),
            I64(num)    => num.to_string(),
            I128(num)   => num.to_string(),
            F64(num)    => num.to_string(),
            Bool(bool)  => bool.to_string(),
            Str(str)    => str.clone(),
            FnNative(_) => "<native>".to_string(),
            FnBridge(_) => "<bridge>".to_string(),
            Nil()       => "nil".to_string(),
        }
    }

    /// Returns the type of object as String
    pub fn to_type_string(&self) -> String {
        match self {
            Sym(_)      => "quote",
            Lst(_)      => "list",
            I32(_)      => "i32",
            I64(_)      => "i64",
            I128(_)     => "i128",
            F64(_)      => "f64",
            Bool(_)     => "bool",
            Str(_)      => "string",
            FnNative(_) => "native",
            FnBridge(_) => "bridge",
            Nil()       => "nil",
        }
        .to_string()
    }

    /// Converts a String into a `Obj::Str` by 
    /// transforming `"xxx"` into `Obj::Str(xxx)`
    /// ## Note
    /// if the given input cannot be converted, returns `None`
    pub fn sym_to_str(str: &String) -> Option<Obj> {
        // asserts string begins and ends with ""
        if &str[0..1] != "\"" || &str[str.len() - 1..] != "\"" {
            return None
        }
    
        // collects the string content 
        let col = {
            str
                .chars()
                .skip(1)
                .take(str.len() - 2)
                .collect::<String>()
        };
    
        Some(Str(col))
    }
       
    /// Converts a String into an `Obj::I32|I64|I128|F64`
    /// ### Delimiter
    /// Number can be separated by '_'
    /// - i.e. `100_000`  `-1_0_0_0_0_0.`  `+_100000.__123__`
    /// ### Binary
    /// Numbers can be interpreted as binary
    /// - i.e. `#b0001_1010_0100`
    /// ### Hexadecimal
    /// Numbers can be interpreted as hexadecimal
    /// - i.e. `#h6_68A0`
    /// ### Typing
    /// The num-type (`i32|i64|i128|f64`) is chosen
    /// chosen depending on the size of the number
    /// ## Error
    /// * Incorrect form : `Err(MisForm)`
    /// * Numeral overflow: `Err(OverFlow)`
    pub fn sym_to_num(str: &String) -> Err<Obj> {
        if str.is_empty() {
            return Err(MisForm)
        }

        // index of first digit
        let mut fst_dig = None;
        // index of number beginning
        let mut num_beg = None;
        // index of number end
        let mut num_end = 0;
        // index of decimal point
        let mut decm_loc = None;
        // index of the sign
        let mut sign_loc = None;
        // sign multiplier (-/+)
        let mut sign = 1;

        let base;
        if str.len() < 2 {
            base = 10;
        }
        else {
            match &str[0..2] {
                "#b" => base = 2,  // binary
                "#h" => base = 16, // hexadecimal
                _ => base = 10,
            };
        }

        for (i, ch) in str.chars().enumerate() {
            if ch.is_digit(base) || ch == '.' || ch == '_' || ch == '+' || ch == '-' { 
                if ch.is_digit(base) && fst_dig.is_none() {
                    fst_dig = Some(i);
                }      
                if num_beg.is_none() {
                    num_beg = Some(i);
                }

                num_end = i;

                match ch {
                    '.' => {
                        if decm_loc.is_some() || base != 10 {
                            return Err(MisForm)
                        }
                        decm_loc = Some(i);
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
            }
        }

        // assert a first digit was found
        if fst_dig.is_none() || num_beg.is_none() || (num_end + 1) as usize != str.len() {
            return Err(MisForm)
        }
        
        // unwrap data
        let fst_dig = fst_dig.unwrap();

        // filter and collect into numerics
        let digits = {
            str
                .chars()
                .filter(|ch| ch.is_digit(base))
                .map(|dig| dig.to_digit(base).unwrap() as i128)
                .collect::<Vec<i128>>()
        };

        // sum digits^base(x) 
        let num = {
            digits
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, dig)| 
                    acc + dig * (base as i128).pow(i as u32))
        };

        if let Some(loc) = decm_loc {
            let num = num as f64 / (base as f64).powi((digits.len() - loc + fst_dig) as i32);
            Ok(F64(sign as f64 * num))
        }
        else {
            if let Ok(int32) = i32::try_from(sign * num) {
                Ok(I32(int32))
            }
            else if let Ok(int64) = i64::try_from(sign * num) {
                Ok(I64(int64))
            }
            else if let Ok(int128) = i128::try_from(sign * num) {
                Ok(I128(int128))
            }
            else {
                Err(OverFlow)
            }
        }
    }
}