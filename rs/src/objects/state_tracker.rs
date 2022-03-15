use crate::objects::position::Position;
use crate::objects::token::TokenType;

#[derive(Clone)]
pub struct StateTracker {
    pub position: Position,
    pub is_literal_string: bool,
    pub literal_string_type: TokenType,
    pub prev_type: TokenType,
}

impl Default for StateTracker {
    fn default() -> Self {
        StateTracker {
            position: Position::default(),
            is_literal_string: false,
            literal_string_type: TokenType::Null,
            prev_type: TokenType::Null,
        }
    }
}
