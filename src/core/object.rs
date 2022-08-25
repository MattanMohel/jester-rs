use std::ops::Mul;

use super::{
    rc_cell::RcCell, 
    node::Node
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

pub fn string_to_num(str: String) -> Option<Obj> {
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
        str.chars()
            .skip(num_beg as usize)
            .take(num_len as usize)
            .filter(|ch| ch.is_digit(base))
            .map(|dig| dig.to_digit(base).unwrap() as i128)
            .collect::<Vec<i128>>();

    // sum digits^base(x) 
    let num =
        digits.iter()
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
