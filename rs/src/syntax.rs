use std::collections::HashMap;
use derivative::Derivative;
use crate::lexer::StateTracker;

pub struct Token {
    pub(crate) value: String,
    pub(crate) type_: TokenType,
    pub(crate) line: i32,
    pub(crate) column: i32,
    pub(crate) categories: Vec<TokenCategory>
}

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
    pub(crate) type_: TokenType,
    #[derivative(Default(value = "|states: &StateTracker| -> bool { !states.is_literal_string }"))]
    pub(crate) condition: &'a dyn Fn(&StateTracker) -> bool,
    #[derivative(Default(value = "|states: &mut StateTracker| {}"))]
    pub(crate) state_changes: &'a dyn FnMut(&mut StateTracker),
    #[derivative(Default(value = ""))]
    pub(crate) prohibited: String,
    #[derivative(Default(value = ""))]
    pub(crate) next_prohibited: String,
    pub(crate) match_whole: bool,
    pub(crate) categories: Vec<TokenCategory>
}

pub(crate) const TOKEN_CATALOGUE: HashMap<&str, TokenEntry> = HashMap::from([
    ("//", TokenEntry{
        type_: TokenType::CommentStart,
        condition: &|states: &StateTracker| -> bool {states.prev_type != TokenType::CommentStart},
        state_changes: &|states: &mut StateTracker| {
            states.is_literal_string = true;
            states.literal_string_type = TokenType::Comment;
        },
        prohibited: "".to_string(),
        next_prohibited: "".to_string(),
        match_whole: false,
        categories: vec![TokenCategory::LiteralStringStart]
    })
]);