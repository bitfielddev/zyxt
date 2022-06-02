use crate::Type;

#[derive(Debug)]
pub enum OprError {
    TypecastError(Type),
    NoImplForOpr
}