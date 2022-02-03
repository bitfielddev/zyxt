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

pub struct TokenEntry<'a> {
    pub value: &'a str,
    pub type_: TokenType,
    pub condition: &'a dyn Fn(&StateTracker) -> bool,
    pub state_changes: &'a dyn Fn(&StateTracker) -> StateTracker,
    pub prohibited: &'a str,
    pub next_prohibited: &'a str,
    pub match_whole: bool,
    pub categories: &'a [TokenCategory]
}
impl Default for TokenEntry<'static> {
    fn default() -> Self {
        TokenEntry {
            value: "",
            type_: TokenType::Null,
            condition: &|states| { !states.is_literal_string },
            state_changes: &|states| { states.clone() },
            prohibited: "",
            next_prohibited: "",
            match_whole: false,
            categories: &[]
        }
    }
}

fn get_token_entry<'a>(stack: &Vec<String>, states: &'a StateTracker, input: &String) -> Option<(String, TokenEntry<'static>)>