use std::ops::Mul;

use super::{
    rc_cell::RcCell, 
    node::Node, type_id::TypeId
};

use Obj::*;

#[derive(Clone)]
pub enum Obj {
    Sym(RcCell<Obj>),
    Lst(Node),
    
    I32(i32),
    I64(i64),
    I128(i128),

    F64(f64),

    Bool(bool),
    Str(String),
    Nil(),
}

impl From<String> for Obj {
    fn from(str: String) -> Self {
        if let Some(num) = to_num_obj(&str) {
            num
        }
        else if let Some(str) = to_str_obj(&str) {
            str
        }
        else {
            Obj::Nil()
        }
    }
}

pub fn to_str_obj(str: &String) -> Option<Obj> {
    // asserts string begins and ends with ""
    if &str[0..1] != "\"" || &str[str.len() - 1..] != "\"" {
        return None
    }

    // collects the string without the ""
    let col =
        str
            .chars()
            .skip(1)
            .take(str.len() - 2)
            .collect::<String>();

    Some(Str(col))
}

/// Converts a `String` into a num `Obj`
/// * Able to convert any numeral type which can
///   be separated by '_'
///   * i.e. `100_000` , `-100_000.` , `+100_000.123`
/// * Able to interperet binary numerals
///   * i.e. `#b0001_1010_0100`
/// * Able to interperet hexadecimal numerals
///   * i.e. `#h6_68A0`
/// * The num-type ( `i32` | `i64` | `i128` | `f64` ) is 
///   chosen dynamically depending on the necessary size
pub fn to_num_obj(str: &String) -> Option<Obj> {
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
    match &str[0..2] {
        "#b" => base = 2,  // binary
        "#h" => base = 16, // hexadecimal
        _ => base = 10,
    };

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
                        return None
                    }
                    decm_loc = Some(i);
                }
                '+' | '-' => {
                    if ch == '-' {
                        sign = -1;
                    }
                    if fst_dig.is_some() || sign_loc.is_some() || base != 10 {
                        return None
                    }
                    sign_loc = Some(i);
                }
                _ => ()
            }
        }
    }

    // assert a first digit was found
    if fst_dig.is_none() || num_beg.is_none() || (num_end + 1) as usize != str.len() {
        return None
    }
    
    // unwrap data
    let fst_dig = fst_dig.unwrap();
    let num_beg = num_beg.unwrap();
    let num_len = num_end - num_beg + 1;

    // filter and collect into numerics
    let digits = 
        str
            .chars()
            .skip(num_beg as usize)
            .take(num_len as usize)
            .filter(|ch| ch.is_digit(base))
            .map(|dig| dig.to_digit(base).unwrap() as i128)
            .collect::<Vec<i128>>();

    // sum digits^base(x) 
    let num =
        digits
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, dig)| {
                acc + dig * (base as i128).pow(i as u32)
            });

    if let Some(loc) = decm_loc {
        let num = num as f64 / (base as f64).powi((num_len - loc + fst_dig) as i32);
        Some(F64(sign as f64 * num))
    }
    else {
        if let Ok(int32) = i32::try_from(sign * num) {
            Some(I32(int32))
        }
        else if let Ok(int64) = i64::try_from(sign * num) {
            Some(I64(int64))
        }
        else {
            Some(I128(sign * num))
        }
    }
}
