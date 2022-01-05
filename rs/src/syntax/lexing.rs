use std::fmt::{Display, Formatter, Result};
use crate::errors;
use crate::lexer::StateTracker;
use crate::syntax::parsing::{Flag, OprType};

#[derive(Clone, PartialEq)]
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
    AssignmentOpr(OprType), // =, +=, etc
    ArithmeticBitwiseOpr(OprType), // +, -, /f, rt, \& etc
    RelationalOpr(OprType), // ==, >, is etc
    LogicalOpr(OprType), // &&, ||, ^^ etc
    ConcatOpr, // ..
    SwapOpr, // ><
    TypeOpr, // istype, isnttype etc
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

pub fn token_catalogue() -> Vec<TokenEntry<'static>> {vec![
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
        value: ":=",
        type_: TokenType::DeclarationStmt,
        ..Default::default()
    },
    TokenEntry{
        value: "+",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Plus),
        next_prohibited: r"[^+\-=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Minus),
        next_prohibited: r"[^+\-=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "+-",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::PlusMinus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-+",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::MinusPlus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "±",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::PlusMinus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "∓",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::MinusPlus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "·",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::DotMult),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "*",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::AstMult),
        next_prohibited: r"[^=/]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "×",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::CrossMult),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::FractDiv),
        next_prohibited: r"[^fc~=*/]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Div),
        next_prohibited: r"[^fc~=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/f",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::FloorfractDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/c",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::CeilfractDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/~",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::RoundfractDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷f",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::FloorDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷c",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::CeilDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "÷~",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::RoundDiv),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "^",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Power),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "%",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Modulo),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "rt",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Root),
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "lg",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Logarithm),
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\&",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::BitAnd),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\|",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::BitOr),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\^",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::BitXor),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\<<",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::BitLshift),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::BitRshift),
        next_prohibited: r"[^=>]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>>",
        type_: TokenType::ArithmeticBitwiseOpr(OprType::Bit0Rshift),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "=",
        type_: TokenType::AssignmentOpr(OprType::Null),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "+=",
        type_: TokenType::AssignmentOpr(OprType::Plus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "-=",
        type_: TokenType::AssignmentOpr(OprType::Minus),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "*=",
        type_: TokenType::AssignmentOpr(OprType::AstMult),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/=",
        type_: TokenType::AssignmentOpr(OprType::FractDiv),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/f=",
        type_: TokenType::AssignmentOpr(OprType::FloorfractDiv),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/c=",
        type_: TokenType::AssignmentOpr(OprType::CeilfractDiv),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "/~=",
        type_: TokenType::AssignmentOpr(OprType::RoundfractDiv),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "%=",
        type_: TokenType::AssignmentOpr(OprType::Modulo),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\&=",
        type_: TokenType::AssignmentOpr(OprType::BitAnd),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\|=",
        type_: TokenType::AssignmentOpr(OprType::BitOr),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\^=",
        type_: TokenType::AssignmentOpr(OprType::BitXor),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\<<=",
        type_: TokenType::AssignmentOpr(OprType::BitLshift),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>=",
        type_: TokenType::AssignmentOpr(OprType::BitRshift),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\>>>=",
        type_: TokenType::AssignmentOpr(OprType::Bit0Rshift),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "==",
        type_: TokenType::RelationalOpr(OprType::Eq),
        next_prohibited: r"[^=<]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: ">",
        type_: TokenType::RelationalOpr(OprType::Gt),
        next_prohibited: r"[^=<]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "<",
        type_: TokenType::RelationalOpr(OprType::Lt),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: ">=",
        type_: TokenType::RelationalOpr(OprType::Gteq),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "<=",
        type_: TokenType::RelationalOpr(OprType::Lteq),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!=",
        type_: TokenType::RelationalOpr(OprType::Noteq),
        next_prohibited: r"[^=]",
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "===",
        type_: TokenType::RelationalOpr(OprType::Iseq),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!==",
        type_: TokenType::RelationalOpr(OprType::Isnteq),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "is",
        type_: TokenType::RelationalOpr(OprType::Is),
        next_prohibited: r"[^tn]",
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "isnt",
        type_: TokenType::RelationalOpr(OprType::Isnt),
        next_prohibited: r"[^t]",
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "&&",
        type_: TokenType::LogicalOpr(OprType::And),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "||",
        type_: TokenType::LogicalOpr(OprType::Or),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "^^",
        type_: TokenType::LogicalOpr(OprType::Xor),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "istype",
        type_: TokenType::RelationalOpr(OprType::Istype),
        match_whole: true,
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "isnttype",
        type_: TokenType::RelationalOpr(OprType::Isnttype),
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
        type_: TokenType::UnaryOpr(OprType::Increment, UnarySide::Right),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "--",
        type_: TokenType::UnaryOpr(OprType::Decrement, UnarySide::Right),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "\\~",
        type_: TokenType::UnaryOpr(OprType::BitComplement, UnarySide::Left),
        categories: &[TokenCategory::Operator],
        ..Default::default()
    },
    TokenEntry{
        value: "!",
        type_: TokenType::UnaryOpr(OprType::Not, UnarySide::Left),
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
        type_: TokenType::Flag(Flag::Hoi),
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "pub",
        type_: TokenType::Flag(Flag::Pub),
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "priv",
        type_: TokenType::Flag(Flag::Priv),
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "prot",
        type_: TokenType::Flag(Flag::Prot),
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "const",
        type_: TokenType::Flag(Flag::Const),
        match_whole: true,
        next_prohibited: r"\s",
        ..Default::default()
    },
    TokenEntry{
        value: "true",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        categories: &[TokenCategory::Literal],
        ..Default::default()
    },
    TokenEntry{
        value: "false",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        categories: &[TokenCategory::Literal],
        ..Default::default()
    },
    TokenEntry{
        value: "null",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        categories: &[TokenCategory::Literal],
        ..Default::default()
    },
    TokenEntry{
        value: "inf",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        categories: &[TokenCategory::Literal],
        ..Default::default()
    },
    TokenEntry{
        value: "undef",
        type_: TokenType::LiteralMisc,
        match_whole: true,
        next_prohibited: r"\s",
        categories: &[TokenCategory::Literal],
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
        next_prohibited: r"[^=]",
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
        type_: TokenType::Variable,
        prohibited: r"\W",
        next_prohibited: r"[\W\s]",
        ..Default::default()
    }
]}