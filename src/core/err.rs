use std::{error::Error, fmt::Display};

pub trait AsResult 
where Self: Sized 
{
    fn ok_then<O, E>(&self, ok: O, err: E) -> Result<O, E>;
    fn ok_or<E>(&self, err: E) -> Result<(), E>;
}

impl AsResult for bool {
    fn ok_then<O, E>(&self, ok: O, err: E) -> Result<O, E> {
        if *self {
            Ok(ok)
        }
        else {
            Err(err)
        }
    }

    fn ok_or<E>(&self, err: E) -> Result<(), E> {
        if *self {
            Ok(())
        }
        else {
            Err(err)
        }
    }
}

pub type Err<T = ()> = Result<T, ErrType>;

#[derive(Debug)]
pub enum ErrType {
    NonSym,
    DupSym,
    NonMod,
    DupMod, 
    MisType,
    ErrCast,
    ErrList,
    Unbalanced,  
    OutOfBound,
}

impl Error for ErrType {}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error")
    }
}
