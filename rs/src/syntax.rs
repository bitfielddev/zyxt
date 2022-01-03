pub struct Token {
    value: string,
    r#type: TokenType,
    line: i32,
    column: i32,
    categories: [TokenCategory]
}

pub enum TokenType {
    comment_start, // //
    comment_end, // \n
    multiline_comment_start, // /*
    multiline_comment_end, // */
    flag, // hoi, pub, priv, prot, const
    unary_opr, // \~, ++, ! etc
    assignment_opr, // =, +=, etc
    arithmetic_bitwise_opr, // +, -, /f, rt, \& etc
    relational_opr, // ==, >, is etc
    logical_opr, // &&, ||, ^^ etc
    concat_opr, // ..
    swap_opr, // ><
    type_opr, // istype, isnttype etc
    dot_opr, // .
    literal_misc, // true, null, etc
    literal_number, // 3, 24, -34.5 etc
    literal_string, // "abc" etc
    statement_end, // ;
    open_paren, // (
    close_paren, // )
    open_square_paren, // [
    close_square_paren, // ]
    open_curly_paren, // {
    close_curly_paren, // }
    open_angle_paren, // <
    close_angle_paren, // >
    comma, // ,
    colon, // :
    comment,
    variable,
    null
}

pub enum TokenCategory {
    operator,
    literal,
    parenthesis,
    open_paren,
    close_paren,
    literal_string_start, //  marks the start of a literal string
    literal_string_end // marks the end of a literal string
}

