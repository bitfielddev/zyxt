use crate::{errors::ZError, types::token::TokenType};

impl ZError {
    #[must_use]
    pub fn p001() -> Self {
        Self::new("P001", "Stray start/end of comment".to_owned())
    }
    #[must_use]
    pub fn p002() -> Self {
        Self::new(
            "P002",
            "Unexpected token (could not be collapsed)".to_owned(),
        )
    }
    #[must_use]
    pub fn p003() -> Self {
        Self::new(
            "P003",
            "Unexpected token (could not be parsed into AST)".to_owned(),
        )
    }
    #[must_use]
    pub fn p004() -> Self {
        Self::new(
            "P004",
            "Missing ident before assignment operator".to_owned(),
        )
    }
    #[must_use]
    pub fn p005() -> Self {
        Self::new("P005", "Missing value after assignment operator".to_owned())
    }
    #[must_use]
    pub fn p006() -> Self {
        Self::new(
            "P006",
            "Missing value to left/right of binary operator".to_owned(),
        )
    }
    #[must_use]
    pub fn p007() -> Self {
        Self::new("P007", "Expected more after this token".to_owned())
    }
    #[must_use]
    pub fn p008() -> Self {
        Self::new("P008", "Expected more before this token".to_owned())
    }
    #[must_use]
    pub fn p009(tot: TokenType) -> Self {
        Self::new("P009", format!("Expected closing {tot:?}"))
    }
    #[must_use]
    pub fn p010() -> Self {
        Self::new("P010", "Classes cannot have parameters here".to_owned())
    }
    #[must_use]
    pub fn p011() -> Self {
        Self::new(
            "P011",
            "Classes must have a block after `class` (consider using a `struct`)".to_owned(),
        )
    }
    #[must_use]
    pub fn p012() -> Self {
        Self::new("P012", "Invalid ident name".to_owned())
    }
    #[must_use]
    pub fn p013() -> Self {
        Self::new(
            "P013",
            "Invalid tokens between flag and declared ident".to_owned(),
        )
    }
    #[must_use]
    pub fn p014() -> Self {
        Self::new("P014", "Cannot delete a dereferenced ident".to_owned())
    }
    #[must_use]
    pub fn p015() -> Self {
        Self::new("P015", "Only idents can be deleted".to_owned())
    }
    #[must_use]
    pub fn p016(tok: &'static str) -> Self {
        Self::new("P016", format!("{tok} not after `if`"))
    }
    #[must_use]
    pub fn p017(tok: &'static str) -> Self {
        Self::new("P017", format!("{tok} found after `else`"))
    }
    #[must_use]
    pub fn p018() -> Self {
        Self::new(
            "P018",
            "Block expected after condition expression".to_owned(),
        )
    }
    #[must_use]
    pub fn p019() -> Self {
        Self::new("P019", "Expected an ident as an argument name".to_owned())
    }
    #[must_use]
    pub fn p020() -> Self {
        Self::new("P020", "Expected an expression as a type".to_owned())
    }
    #[must_use]
    pub fn p021() -> Self {
        Self::new(
            "P021",
            "Detected unparenthesised argument list with no function".to_owned(),
        )
    }
    #[must_use]
    pub fn p022() -> Self {
        Self::new("P022", "Expected expression before `.`".to_owned())
    }
    #[must_use]
    pub fn p023() -> Self {
        Self::new("P023", "Stray `)`".to_owned())
    }
    #[must_use]
    pub fn p024() -> Self {
        Self::new("P024", "Stray `(`".to_owned())
    }
}
