use crate::Type;

pub enum OprError {
    TypecastError(Type),
    NoImplForOpr
}