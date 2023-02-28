use std::fmt::{Debug, Display};

use crate::{errors::ZError, types::typeobj::Type};

impl ZError {
    #[must_use]
    pub fn t001() -> Self {
        Self::new("T001", "Constants are not mutable".to_owned())
    }
    #[must_use]
    pub fn t002(sym: &str) -> Self {
        Self::new("T002", format!("Symbol `{sym}` is not defined"))
    }
    #[must_use]
    pub fn t003<T: Clone + PartialEq + Debug>(block_type: &Type<T>, return_type: &Type<T>) -> Self {
        Self::new("T003", format!("Block returns variable of type `{block_type}` earlier on, but also returns variable of type `{return_type}`"))
    }
    #[must_use]
    pub fn t004<T: Clone + PartialEq + Debug>(expected: &Type<T>, actual: &Type<T>) -> Self {
        Self::new(
            "T004",
            format!("Procedure/function expected argument of type `{expected}`, got `{actual}`"),
        )
    }
    #[must_use]
    pub fn t005(ty: &Type<impl Clone + PartialEq + Debug>, attr: impl Display) -> Self {
        Self::new(
            "T005",
            format!("Symbol of type `{ty}` has no attribute `{attr}`"),
        )
    }
    #[must_use]
    pub fn t006() -> Self {
        Self::new("T006", "Expected a pattern".to_owned())
    }
    #[must_use]
    pub fn t007() -> Self {
        Self::new("T007", "Expected a type".to_owned())
    }
    #[must_use]
    pub fn t008() -> Self {
        Self::new("T008", "Expected an ident".to_owned())
    }
    #[must_use]
    pub fn t009<T: Clone + PartialEq + Debug>(expected: &Type<T>, actual: &Type<T>) -> Self {
        Self::new(
            "T009",
            format!(
                "Procedure/function expected return value of type `{expected}`, got `{actual}`"
            ),
        )
    }
    #[must_use]
    pub fn t010<T: Clone + PartialEq + Debug>(expected: &Type<T>, actual: &Type<T>) -> Self {
        Self::new(
            "T010",
            format!("Attempt to assign value of type `{actual}` to symbol of type `{expected}`"),
        )
    }
}
