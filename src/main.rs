mod core;

use crate::core::object::{Obj, str_to_num_obj};

fn main() {
    let test = ["#b101101", "#hDD6FF", "1003987", "-213897.388", "+823778.11", "0.00", "1_000_000"];

    for str in test.iter() {
        match to_num_obj(&str.to_string()) {
            Some(num) => {
                match num {
                    Obj::F64(f) => println!("f32: {}", f),
                    Obj::I32(i) => println!("i32: {}", i),
                    Obj::I64(i) =>  println!("i64: {}", i),
                    Obj::I128(i) =>  println!("i128: {}", i),
                    _ => ()
                }
            },
            None => println!("not a number")
        }
    }
}
