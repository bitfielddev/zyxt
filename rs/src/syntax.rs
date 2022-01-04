use std::collections::HashMap;

pub struct Token {
    value: string,
    r#type: TokenType,
    line: i32,
    column: i32,
    categories: [TokenCategory]
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

pub enum TokenCategory {
    Operator,
    Literal,
    Parenthesis,
    OpenParen,
    CloseParen,
    LiteralStringStart, //  marks the start of a literal string
    LiteralStringEnd // marks the end of a literal string
}

#[derive(Derivative)]
pub struct TokenEntry {
    pub(crate) type_: TokenType,
    #[derivative(Default(value = |states: &StateTracker| { !states.is_literal_string }))]
    condition: dyn Fn(StateTracker) -> bool,
    #[derivative(Default(value = |states: &StateTracker| {}))]
    state_changes: dyn Fn(StateTracker),
    pub(crate) prohibited: String,
    pub(crate) next_prohibited: String,
    pub(crate) match_whole: bool,
    pub(crate) categories: [TokenCategory]
}

pub(crate) const TOKEN_CATALOGUE: HashMap<&str, TokenEntry> = HashMap::from([
    ("//", TokenEntry(
        type_ = TokenType::CommentStart,
        condition = |states| {states.prev_type != TokenType::CommentStart},
        state_changes = |states| {
            states.is_literal_string = true;
            states.literal_string_type = TokenType::Comment;
        },
        categories: [TokenCategory::LiteralStringStart]))
]);