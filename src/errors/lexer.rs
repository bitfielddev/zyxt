use crate::errors::ZError;

impl ZError {
    #[must_use]
    pub fn l001() -> Self {
        Self::new(
            "L001",
            "Unexpected ident (Lexer didn't recognise)".to_owned(),
        )
    }
    #[must_use]
    pub fn l002() -> Self {
        Self::new("L002", "Unexpected end of comment".to_owned())
    }
}
