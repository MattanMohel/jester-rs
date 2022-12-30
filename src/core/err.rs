use std::{
    error::Error,
    fmt::Display,
    io
};

pub type Err<T = ()> = Result<T, ErrType>;

#[derive(Debug)]
pub enum ErrType {
    IoErr,
    NonSym,
    DupSym,
    NonMod,
    DupMod, 
    MisType,
    MisComp,
    MisForm,
    ErrCast,
    ErrList,
    Overflow,
    Unbalanced,  
    OutOfBound,
    RuntimeAssert
}

impl Error for ErrType {}

impl Display for ErrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error")
    }
}

impl From<io::Error> for ErrType {
    fn from(err: io::Error) -> Self {
        ErrType::IoErr
    }
}