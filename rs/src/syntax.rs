use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use crate::errors;
use crate::lexer::StateTracker;

/* === TOKEN === */

#[derive(Clone)]
pub struct Token {
    pub(crate) value: String,
    pub(crate) type_: TokenType,
    pub(crate) line: u32,
    pub(crate) column: u32,
    pub(crate) categories: &'static [TokenCategory]
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token[value=\"{}\", type={:?}, line={}, column={}, categories={:?}]",
               self.value, self.type_, self.line, self.column, self.categories)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub(crate) value: &'a str,
    pub(crate) type_: TokenType,
    pub(crate) condition: &'a dyn Fn(&StateTracker) -> bool,
    pub(crate) state_changes: &'a dyn Fn(&StateTracker) -> StateTracker,
    pub(crate) prohibited: &'a str,
    pub(crate) next_prohibited: &'a str,
    pub(crate) match_whole: bool,
    pub(crate) categories: &'a [TokenCategory]
}
impl Default for TokenEntry<'static> {
    fn default() -> Self {
        TokenEntry {
            value: "",
            type_: TokenType::Null,
            condition: &|states| { !states.is_literal_string },
            state_changes: &|states| {states.clone()},
            prohibited: "",
            next_prohibited: "",
            match_whole: false,
            categories: &[]
        }
    }
}

pub(crate) fn token_catalogue() -> Vec<TokenEntry<'static>> {vec![
    TokenEntry{
        value: "//",
        type_: TokenType::CommentStart,
        condition: &|states| {states.prev_type != TokenType::CommentStart},
        state_changes: &|states| {
            let mut new_states = states.clone();
            new_states.is_literal_string = true;
            new_states.literal_string_type = TokenType::Comment;
            new_states
        },
        categories: &[TokenCategory::LiteralStringStart],
        ..Default::default()
    },
    TokenEntry{
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
    },
    TokenEntry{
        value: "/*",
        type_: TokenType::MultilineCommentStart,
        condition: &|states| {states.prev_type != TokenType::MultilineCommentStart},
        state_changes: &|states| {
            let mut new_states = states.clone();
            new_states.is_literal_string = true;
            new_states.literal_string_type = TokenType::Comment;
            new_states
        },
        categories: &[TokenCategory::LiteralStringStart],
        ..Default::default()
    },
    TokenEntry{
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
    },
    TokenEntry{
        value: "+",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^+\-=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^+\-=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "+-",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-+",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "±",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "∓",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "·",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "*",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=/]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "×",
        type_: TokenType::ArithmeticBitwiseOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^fc~=*/]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^fc~=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/f",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/c",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/~",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷f",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷c",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷~",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "^",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "rt",
        type_: TokenType::ArithmeticBitwiseOpr,
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "lg",
        type_: TokenType::ArithmeticBitwiseOpr,
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\&",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\|",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\^",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\<<",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=>]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>>",
        type_: TokenType::ArithmeticBitwiseOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "=",
        type_: TokenType::AssignmentOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "+=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "*=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/f=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/c=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/~=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "%=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\&=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\|=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\^=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\<<=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>>=",
        type_: TokenType::AssignmentOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "==",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^=<]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: ">",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^=<]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "<",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: ">=",
        type_: TokenType::RelationalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "<=",
        type_: TokenType::RelationalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!=",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "===",
        type_: TokenType::RelationalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!==",
        type_: TokenType::RelationalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "is",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^tn]",
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "isnt",
        type_: TokenType::RelationalOpr,
        next_prohibited: r"[^t]",
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "&&",
        type_: TokenType::LogicalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "||",
        type_: TokenType::LogicalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "^^",
        type_: TokenType::LogicalOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "istype",
        type_: TokenType::RelationalOpr,
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "isnttype",
        type_: TokenType::RelationalOpr,
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "><",
        type_: TokenType::SwapOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "..",
        type_: TokenType::ConcatOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "++",
        type_: TokenType::UnaryOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "--",
        type_: TokenType::UnaryOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\~",
        type_: TokenType::UnaryOpr,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!",
        type_: TokenType::UnaryOpr,
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "(",
        type_: TokenType::OpenParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            new_states.brackets.push('(');
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
        ..Default::default()
    },
    TokenEntry{
        value: "[",
        type_: TokenType::OpenParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            new_states.brackets.push('[');
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
        ..Default::default()
    },
    TokenEntry{
        value: "{",
        type_: TokenType::OpenParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            new_states.brackets.push('{');
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
        ..Default::default()
    },
    TokenEntry{
        value: ")",
        type_: TokenType::CloseParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            if states.brackets.len() == 0 {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_2(')');
            } else if states.brackets.last().unwrap() != &'(' {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_0(')', *states.brackets.last().unwrap());
            }
            new_states.brackets.pop();
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
        ..Default::default()
    },
    TokenEntry{
        value: "]",
        type_: TokenType::CloseParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            if states.brackets.len() == 0 {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_2(']');
            } else if states.brackets.last().unwrap() != &'[' {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_0(']', *states.brackets.last().unwrap());
            }
            new_states.brackets.pop();
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
        ..Default::default()
    },
    TokenEntry{
        value: "}",
        type_: TokenType::CloseParen,
        state_changes: &|states| {
            let mut new_states = states.clone();
            if states.brackets.len() == 0 {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_2('}');
            } else if states.brackets.last().unwrap() != &'{' {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_0('}', *states.brackets.last().unwrap());
            }
            new_states.brackets.pop();
            new_states
        },
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
        ..Default::default()
    },
    TokenEntry{
        value: ".",
        type_: TokenType::DotOpr,
        next_prohibited: r"[^\.]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "hoi",
        type_: TokenType::Flag,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "pub",
        type_: TokenType::Flag,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "priv",
        type_: TokenType::Flag,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "prot",
        type_: TokenType::Flag,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "const",
        type_: TokenType::Flag,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "true",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "false",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "null",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "inf",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "undef",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: ";",
        type_: TokenType::StatementEnd,
        state_changes: &|states| {
            if states.brackets.len() != 0 && ['(', '['].contains(states.brackets.last().unwrap()) {
                errors::error_pos(&states.position.filename, states.position.line, states.position.column);
                errors::error_2_0_1(*states.brackets.last().unwrap());
            }
            states.clone()
        },
        ..Default::default()
    },
    TokenEntry {
        value: ",",
        type_: TokenType::Comma,
        ..Default::default()
    },
    TokenEntry {
        value: ":",
        type_: TokenType::Colon,
        ..Default::default()
    },
    TokenEntry {
        type_: TokenType::LiteralNumber,
        prohibited: r"\D",
        next_prohibited: r"\D",
        categories: &[TokenCategory::Literal],
        ..Default::default()
    },
    TokenEntry {
        prohibited: r"\W",
        next_prohibited: r"[\W\s]",
        ..Default::default()
    }
]}

/* === PARSER === */
pub(crate) enum OprType {
    Increment,
    Decrement,
    PlusSign,
    MinusSign,
    Not,
    BitComplement,
    Logarithm,
    Root,
    Power,
    DotMult,
    AstMult,
    CrossMult,
    Div,
    FloorDiv,
    CeilDiv,
    RoundDiv,
    FractDiv,
    FloorfractDiv,
    CeilfractDiv,
    RoundfractDiv,
    Modulo,
    Plus,
    Minus,
    PlusMinus,
    MinusPlus,
    BitLshift,
    BitRshift,
    Bit0Rshift,
    And,
    Or,
    Xor,
    Gt,
    Lt,
    Gteq,
    Lteq,
    Eq,
    Noteq,
    Istype,
    Isnttype,
    Is,
    Isnt,
    Iseq,
    Isnteq,
    BitAnd,
    BitOr,
    BitXor,
    Concat,
    Swap,
    Null
}
pub(crate) const UNARYOPRMAP: HashMap<&str, OprType> = HashMap::from([
    ("++", OprType::Increment),
    ("--", OprType::Decrement),
    ("+", OprType::PlusSign),
    ("-", OprType::MinusSign),
    ("!", OprType::Not),
    ("\\~", OprType::BitComplement)
]);
pub(crate) const BINARYOPRMAP: HashMap<&str, OprType> = HashMap::from([
    ("lg": OprType::Logarithm),
    ("rt", OprType::Root),
    ("^", OprType::Power),
    ("·", OprType::DotMult),
    ("*", OprType::AstMult),
    ("×", OprType::CrossMult),
    ("÷", OprType::Div),
    ("÷f", OprType::FloorDiv),
    ("÷c", OprType::CeilDiv),
    ("÷~", OprType::RoundDiv),
    ("/", OprType::FractDiv),
    ("/c", OprType::FloorfractDiv),
    ("/f", OprType::CeilfractDiv),
    ("/~", OprType::RoundfractDiv),
    ("%", OprType::Modulo),
    ("+", OprType::Plus),
    ("-", OprType::Minus),
    ("+-", OprType::PlusMinus),
    ("-+", OprType::MinusPlus),
    ("±", OprType::PlusMinus),
    ("∓", OprType::MinusPlus),
    ("\\<<", OprType::BitLshift),
    ("\\>>", OprType::BitRshift),
    ("\\>>>", OprType::Bit0Rshift),
    ("&&", OprType::And),
    ("||", OprType::Or),
    ("^^", OprType::Xor),
    (">", OprType::Gt),
    ("<", OprType::Lt),
    ("≥", OprType::Gteq),
    ("≤", OprType::Lteq),
    ("==", OprType::Eq),
    ("!=", OprType::Noteq),
    ("istype", OprType::Istype),
    ("isnttype", OprType::Isnttype),
    ("is", OprType::Is),
    ("isnt", OprType::Isnt),
    ("===", OprType::Iseq),
    ("!==", OprType::Isnteq),
    ("\\&", OprType::BitAnd),
    ("\\|", OprType::BitOr),
    ("\\^", OprType::BitXor),
    ("..", OprType::Concat),
    ("><", OprType::Swap)
]);
pub(crate) const ORDERMAP: HashMap<OprType, i32> = HashMap::from([
    (OprType::Increment, 2),
    (OprType::Decrement, 2),
    (OprType::PlusSign, 2),
    (OprType::MinusSign, 2),
    (OprType::Not, 2),
    (OprType::BitComplement, 2),
    (OprType::Logarithm, 4),
    (OprType::Root, 4),
    (OprType::Power, 3),
    (OprType::DotMult, 5),
    (OprType::AstMult, 6),
    (OprType::CrossMult, 7),
    (OprType::Div, 7),
    (OprType::FloorDiv, 7),
    (OprType::CeilDiv, 7),
    (OprType::RoundDiv, 7),
    (OprType::FractDiv, 6),
    (OprType::FloorfractDiv, 6),
    (OprType::CeilfractDiv, 6),
    (OprType::RoundfractDiv, 6),
    (OprType::Modulo, 6),
    (OprType::Plus, 8),
    (OprType::Minus, 8),
    (OprType::PlusMinus, 8),
    (OprType::MinusPlus, 8),
    (OprType::BitLshift, 9),
    (OprType::BitRshift, 9),
    (OprType::Bit0Rshift, 9),
    (OprType::And, 14),
    (OprType::Or, 16),
    (OprType::Xor, 15),
    (OprType::Gt, 10),
    (OprType::Lt, 10),
    (OprType::Gteq, 10),
    (OprType::Lteq, 10),
    (OprType::Eq, 10),
    (OprType::Noteq, 10),
    (OprType::Istype, 10),
    (OprType::Isnttype, 10),
    (OprType::Is, 10),
    (OprType::Isnt, 10),
    (OprType::Iseq, 10),
    (OprType::Isnteq, 10),
    (OprType::BitAnd, 11),
    (OprType::BitOr, 13),
    (OprType::BitXor, 12),
    (OprType::Concat, 17),
    (OprType::Swap, 19),
]);

pub(crate) enum Flag {Hoi, Pub, Priv, Prot, Const}
pub(crate) const FLAGMAP: HashMap<&str, Flag> = HashMap::from([
    ("hoi", Flag::Hoi),
    ("pub", Flag::Pub),
    ("priv", Flag::Priv),
    ("prot", Flag::Prot),
    ("const", Flag::Const)
]);

pub(crate) struct Statement {
    content: Vec<Token>
}

pub(crate) enum Element {
    BaseElement {
        line: u32,
        column: u32
    },
    Comment {
        line: u32,
        column: u32,
        content: String,
    },
    Call {
        line: u32,
        column: u32,
        content: String,
        called: Element,
        args: Vec<Element>,
        //kwargs
    },
    UnaryOpr {
        line: u32,
        column: u32,
        type_: OprType,
        operand: Element
    },
    BinaryOpr {
        line: u32,
        column: u32,
        type_: OprType,
        operand1: Element,
        operand2: Element
    },
    AssignmentOpr {
        line: u32,
        column: u32,
        variable: Element::Variable,
        content: Element,
        flags: Vec<Flag>,
        type_: Element::Variable,
        operation: OprType
    },
    Literal {
        line: u32,
        column: u32,
        type_: Element::Variable,
        content: String
    },
    Variable {
        line: u32,
        column: u32,
        name: String,
        parent: Element
    },
    NullElement {
        line: u32,
        column: u32
    },
    Token(Token)
}