use std::fmt::{Display, Formatter, Result};
use crate::lexer::StateTracker;
use crate::syntax::parsing::{Flag, OprType};

pub struct Token {
    pub value: String,
    pub type_: TokenType,
    pub line: u32,
    pub column: u32,
    pub categories: &'static [TokenCategory]
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token[value=\"{}\", type={:?}, line={}, column={}, categories={:?}]",
               self.value, self.type_, self.line, self.column, self.categories)
    }
}
impl Default for Token {
    fn default() -> Self {
        Token {
            value: "".to_string(),
            type_: TokenType::Null,
            line: 0,
            column: 0,
            categories: &[]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnarySide { Left, Right }
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    CommentStart, // //
    CommentEnd, // \n
    MultilineCommentStart, // /*
    MultilineCommentEnd, // */
    Flag(Flag), // hoi, pub, priv, prot, const
    UnaryOpr(OprType, UnarySide), // \~, ++, ! etc
    AssignmentOpr(OprType), // =, += etc
    NormalOpr(OprType), // +, -, /f, rt, \&, ==, >, is, &&, ||, ^^, .., ><, istype, isnttype etc
    DotOpr, // .
    DeclarationStmt, // :=
    LiteralMisc, // true, null, etc
    LiteralNumber, // 3, 24, -34.5 etc
    LiteralString, // "abc" etc
    StatementEnd, // ;
    OpenParen, // (
    CloseParen, // )
    OpenSquareParen, // [
    CloseSquareParen, // ]
    OpenCurlyParen, // {
    CloseCurlyParen, // }
    OpenAngleBracket, // <
    CloseAngleBracket, // >
    Comma, // ,
    Colon, // :
    Comment,
    Variable,
    Null
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenCategory {
    Operator,
    Literal,
    Parenthesis,
    OpenParen,
    CloseParen,
    LiteralStringStart, //  marks the start of a literal string
    LiteralStringEnd // marks the end of a literal string
}

pub enum TokenEntry<'a> {
    Singular {
        value: &'a str,
        type_: TokenType,
        re: &'a str,
        categories: &'a [TokenCategory]
    },
    Compound {
        value: &'a str,
        type_: TokenType,
        combination: [TokenType],
        categories: &'a [TokenCategory]
    }
}
impl Default for TokenEntry {
    fn default() -> Self {
        TokenEntry {
            value: "",
            type_: TokenType::Null,
            re: "",
            categories: &[]
        }
    }
}

fn get_possible_tokens<'a>(stack: &Vec<String>, states: &'a StateTracker, input: &String) -> Vec<TokenEntry> {
    if states.is_literal_string {
        return vec![
            TokenEntry::Singular{
                value: "\n",
                type_: TokenType::CommentEnd,
                condition: &|states| {states.prev_type == TokenType::CommentStart},
                state_changes: &|states| {
                    let mut new_states = states.clone();
                    new_states.is_literal_string = false;
                    new_states.literal_string_type = TokenType::Null;
                    new_states
                },
                categories: &[TokenCategory::LiteralStringEnd],
                ..Default::default()
            }, TokenEntry::Singlular{
                value: "*/",
                type_: TokenType::MultilineCommentEnd,
                condition: &|states| {states.prev_type == TokenType::MultilineCommentStart},
                state_changes: &|states| {
                    let mut new_states = states.clone();
                    new_states.is_literal_string = false;
                    new_states.literal_string_type = TokenType::Null;
                    new_states
                },
                categories: &[TokenCategory::LiteralStringEnd],
                ..Default::default()
            }
        ]
    }
    vec![]
}