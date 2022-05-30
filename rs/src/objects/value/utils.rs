macro_rules! as_type {
    ($t:ident, $v:expr) => (
        if let Ok(value) = $v {
            value
        } else {
            return Err(Error::TypecastError);
        }
    )
}

pub enum OprError {
    TypecastError,
    NoImplForOpr
}