use std::fmt::{Display, Formatter, Result};
use derivative::Derivative;
use crate::errors;
use crate::lexer::StateTracker;
use crate::syntax::TokenType::ArithmeticBitwiseOpr;

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

#[derive(Derivative)]
pub struct TokenEntry<'a> {
    pub(crate) value: &'a str,
    pub(crate) type_: TokenType,
    #[derivative(Default(value = "|states| { !states.is_literal_string }"))]
    pub(crate) condition: &'a dyn Fn(&StateTracker) -> bool,
    #[derivative(Default(value = "|states| {}"))]
    pub(crate) state_changes: &'a dyn Fn(&StateTracker) -> StateTracker,
    #[derivative(Default(value = ""))]
    pub(crate) prohibited: &'a str,
    #[derivative(Default(value = ""))]
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
            let mut new_states = states.clone();
            new_states
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