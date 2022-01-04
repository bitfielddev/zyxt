use std::fmt::{Display, Formatter, Result};
use derivative::Derivative;
use crate::lexer::StateTracker;

#[derive(Clone)]
pub struct Token {
    pub(crate) value: String,
    pub(crate) type_: TokenType,
    pub(crate) line: i32,
    pub(crate) column: i32,
    pub(crate) categories: &'static [TokenCategory]
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token[value={}, type={:?}, line={}, column={}, categories={:?}]",
               self.value, self.type_, self.line, self.column, self.categories)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    CommentStart, // //
    CommentEnd, // \n
    MultilineCommentStart, // /*
    MultilineCommentEnd, // */
    Flag, // hoi, pub, priv, prot, const
    UnaryOpr, // \~, ++, ! etc
    AssignmentOpr, // =, +=, etc
    ArithmeticBitwiseOpr, // +, -, /f, rt, \& etc
    RelationalOpr, // ==, >, is etc
    LogicalOpr, // &&, ||, ^^ etc
    ConcatOpr, // ..
    SwapOpr, // ><
    TypeOpr, // istype, isnttype etc
    DotOpr, // .
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

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TokenCategory {
    Operator,
    Literal,
    Parenthesis,
    OpenParen,
    CloseParen,
    LiteralStringStart, //  marks the start of a literal string
    LiteralStringEnd // marks the end of a literal string
}

impl PartialEq for TokenCategory {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Derivative)]
pub struct TokenEntry<'a> {
    pub(crate) value: &'a str,
    pub(crate) type_: TokenType,
    #[derivative(Default(value = "|states| { !states.is_literal_string }"))]
    pub(crate) condition: &'a dyn Fn(&StateTracker) -> bool,
    #[derivative(Default(value = "|states| {}"))]
    pub(crate) state_changes: &'a mut dyn FnMut(&mut StateTracker),
    #[derivative(Default(value = ""))]
    pub(crate) prohibited: String,
    #[derivative(Default(value = ""))]
    pub(crate) next_prohibited: String,
    pub(crate) match_whole: bool,
    pub(crate) categories: &'a [TokenCategory]
}

pub(crate) fn token_catalogue() -> Vec<TokenEntry<'static>> {vec![
    TokenEntry{
        value: "//",
        type_: TokenType::CommentStart,
        condition: &|states| {states.prev_type != TokenType::CommentStart},
        state_changes: &mut|states| {
            states.is_literal_string = true;
            states.literal_string_type = TokenType::Comment;
        },
        prohibited: String::new(),
        next_prohibited: String::new(),
        match_whole: false,
        categories: &[TokenCategory::LiteralStringStart]
    }
]}