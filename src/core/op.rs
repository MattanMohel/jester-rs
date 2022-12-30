use super::{
    obj::Obj::{self, *},
    err::{Err, ErrType::*}
};

impl Obj {
    pub fn add(&mut self, other: Obj) -> Err {
        match self {
            I32(x)  => *x += other.as_i32()?,
            I64(x)  => *x += other.as_i64()?,
            I128(x) => *x += other.as_i128()?,
            F64(x)  => *x += other.as_f64()?,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn sub(&mut self, other: Obj) -> Err {
        match self {
            I32(x)  => *x -= other.as_i32()?,
            I64(x)  => *x -= other.as_i64()?,
            I128(x) => *x -= other.as_i128()?,
            F64(x)  => *x -= other.as_f64()?,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn neg(&mut self) -> Err {
        match self {
            I32(x)  => *x *= -1,
            I64(x)  => *x *= -1,
            I128(x) => *x *= -1,
            F64(x)  => *x *= -1.,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn mul(&mut self, other: Obj) -> Err {
        match self {
            I32(x)  => *x *= other.as_i32()?,
            I64(x)  => *x *= other.as_i64()?,
            I128(x) => *x *= other.as_i128()?,
            F64(x)  => *x *= other.as_f64()?,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn div(&mut self, other: Obj) -> Err {
        match self {
            I32(x)  => *x /= other.as_i32()?,
            I64(x)  => *x /= other.as_i64()?,
            I128(x) => *x /= other.as_i128()?,
            F64(x)  => *x /= other.as_f64()?,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn modulos(&mut self, other: Obj) -> Err {
        match self {
            I32(x)  => *x %= other.as_i32()?,
            I64(x)  => *x %= other.as_i64()?,
            I128(x) => *x %= other.as_i128()?,
            F64(x)  => *x %= other.as_f64()?,
            _ => return Err(MisType)
        }

        Ok(())
    }

    pub fn eq(&self, other: &Obj) -> Err<bool> {
        match (self, other) {
            (Bool(b1), Bool(b2)) => Ok(b1 == b2),
            (Str(s1), Str(s2))   => Ok(s1 == s2),     
            _ => {
                match (self.is_num(), other.is_num()) {
                    (Ok(n1), Ok(n2)) => {
                        Ok(n1 == n2)
                    }
                    _ => Err(MisComp)
                }
            }
        }
    }

    pub fn le(&self, other: &Obj) -> Err<bool> {
        match (self.is_num(), other.is_num()) {
            (Ok(n1), Ok(n2)) => Ok(n1 < n2),
            _ => Err(MisComp)
        }
    }

    pub fn le_eq(&self, other: &Obj) -> Err<bool> {
        match (self.is_num(), other.is_num()) {
            (Ok(n1), Ok(n2)) => Ok(n1 <= n2),
            _ => Err(MisComp)
        }
    }
}