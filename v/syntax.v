/* === TOKEN === */
struct Token {
	value string
	type_ TokenType
	line int
	column int
    categories []TokenCategory
}

pub enum TokenType {
    comment_start // //
    comment_end // \n
    multiline_comment_start // /*
    multiline_comment_end // */
    flag // hoi, pub, priv, prot, const
    unary_opr // \~, ++, ! etc
    assignment_opr // =, +=, etc
    arithmetic_bitwise_opr // +, -, /f, rt, \& etc
    relational_opr // ==, >, is etc
    logical_opr // &&, ||, ^^ etc
    concat_opr // ..
    swap_opr // ><
    type_opr // istype, isnttype etc
    dot_opr // .
    literal_misc // true, null, etc
    literal_number // 3, 24, -34.5 etc
    literal_string // "abc" etc
    statement_end // ;
    open_paren // (
    close_paren // )
    open_square_paren // [
    close_square_paren // ]
    open_curly_paren // {
    close_curly_paren // }
    open_angle_paren // <
    close_angle_paren // >
    comma // ,
    colon // :
    comment
    variable
    null
}

pub enum TokenCategory {
    operator
    literal
    parenthesis
    open_paren
    close_paren
    literal_string_start //  marks the start of a literal string
    literal_string_end // marks the end of a literal string
}

// TODO make functions for literal_strings
struct TokenEntry {
    type_ TokenType // the type of the token
    condition fn (&StateTracker) bool = fn (states &StateTracker) bool {
        return !states.is_literal_string
    } // conditions needed for the token to be valid; do not change states.prev_type that is already handled by the lexer
    state_changes fn (&StateTracker) = fn(states &StateTracker) {} // the state changes that are taken place after the token is validated
    prohibited string // the values for the token to be invalid, given as a regex (if the token is a "")
    next_prohibited string // the values for the next character that are invalid, given as a regex of a single character
    match_whole bool // false: only the end of the stack needs to match; true: the entire stack needs to match
    categories []TokenCategory
}

const token_catalogue = {
    "//": TokenEntry{
        type_: .comment_start
        condition: fn (states &StateTracker) bool {
            return states.prev_type != .comment_start
        }
        state_changes: fn (mut states &StateTracker) {
            states.is_literal_string = true
            states.literal_string_type = .comment
        }
        categories: [.literal_string_start]
    }
    "\n": TokenEntry{
        type_: .comment_end
        condition: fn (states &StateTracker) bool {
            return states.prev_type == .comment_start
        }
        state_changes: fn (mut states &StateTracker) {
            states.is_literal_string = false
            states.literal_string_type = .null
        }
        categories: [.literal_string_end]
    }
    "/*": TokenEntry{
        type_: .multiline_comment_start
        condition: fn (states &StateTracker) bool {
            return states.prev_type != .multiline_comment_start
        }
        state_changes: fn (mut states &StateTracker) {
            states.is_literal_string = true
            states.literal_string_type = .comment
        }
        categories: [.literal_string_start]
    }
    "*/": TokenEntry{
        type_: .multiline_comment_end
        condition: fn (states &StateTracker) bool {
            return states.prev_type == .multiline_comment_start
        }
        state_changes: fn (mut states &StateTracker) {
            states.is_literal_string = false
            states.literal_string_type = .null
        }
        categories: [.literal_string_end]
    }
    "+": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^+\-=]"
        categories: [.operator]
    }
    "-": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^+\-=]"
        categories: [.operator]
    }
    "+-": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "-+": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "±": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "∓": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "·": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "*": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=/]"
        categories: [.operator]
    }
    "×": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "/": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^fc~=*/]"
        categories: [.operator]
    }
    "÷": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^fc~=]"
        categories: [.operator]
    }
    "/f": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "/c": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "/~": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "÷f": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "÷c": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "÷~": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "^": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
    }
    "%": TokenEntry{
        type_: .arithmetic_bitwise_opr
        categories: [.operator]
        next_prohibited: r"[^=]"
    }
    "rt": TokenEntry{
        type_: .arithmetic_bitwise_opr
        match_whole: true
        categories: [.operator]
    }
    "lg": TokenEntry{
        type_: .arithmetic_bitwise_opr
        match_whole: true
        categories: [.operator]
    }
    "\\&": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "\\|": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "\\^": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "\\<<": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "\\>>": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=>]"
        categories: [.operator]
    }
    "\\>>>": TokenEntry{
        type_: .arithmetic_bitwise_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "=": TokenEntry{
        type_: .assignment_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "+=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "-=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "*=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "/=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "/f=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "/c=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "/~=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "%=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\&=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\|=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\^=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\<<=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\>>=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "\\>>>=": TokenEntry{
        type_: .assignment_opr
        categories: [.operator]
    }
    "==": TokenEntry{
        type_: .relational_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    ">": TokenEntry{
        type_: .relational_opr
        next_prohibited: r"[^=<]"
        categories: [.operator]
    }
    "<": TokenEntry{
        type_: .relational_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    ">=": TokenEntry{
        type_: .relational_opr
        categories: [.operator]
    }
    "<=": TokenEntry{
        type_: .relational_opr
        categories: [.operator]
    }
    "!=": TokenEntry{
        type_: .relational_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "===": TokenEntry{
        type_: .relational_opr
        categories: [.operator]
    }
    "!==": TokenEntry{
        type_: .relational_opr
        categories: [.operator]
    }
    "is": TokenEntry{
        type_: .relational_opr
        match_whole: true
        next_prohibited: "[^tn]"
        categories: [.operator]
    }
    "isnt": TokenEntry{
        type_: .relational_opr
        match_whole: true
        next_prohibited: "[^t]"
        categories: [.operator]
    }
    "&&": TokenEntry{
        type_: .logical_opr
        categories: [.operator]
    }
    "||": TokenEntry{
        type_: .logical_opr
        categories: [.operator]
    }
    "^^": TokenEntry{
        type_: .logical_opr
        categories: [.operator]
    }
    "istype": TokenEntry{
        type_: .type_opr
        match_whole: true
        categories: [.operator]
    }
    "isnttype": TokenEntry{
        type_: .type_opr
        match_whole: true
        categories: [.operator]
    }
    "><": TokenEntry{
        type_: .swap_opr
        categories: [.operator]
    }
    "..": TokenEntry{
        type_: .concat_opr
        categories: [.operator]
    }
    "++": TokenEntry{
        type_: .unary_opr
        categories: [.operator]
    }
    "--": TokenEntry{
        type_: .unary_opr
        categories: [.operator]
    }
    "\\~": TokenEntry{
        type_: .unary_opr
        categories: [.operator]
    }
    "!": TokenEntry{
        type_: .unary_opr
        next_prohibited: r"[^=]"
        categories: [.operator]
    }
    "(": TokenEntry{
        type_: .open_paren
        state_changes: fn (mut states &StateTracker) {
            states.brackets << "("
        }
        categories: [.parenthesis, .open_paren]
    }
    "[": TokenEntry{
        type_: .open_square_paren
        state_changes: fn (mut states &StateTracker) {
            states.brackets << "["
        }
        categories: [.parenthesis, .open_paren]
    }
    "{": TokenEntry{
        type_: .open_curly_paren
        state_changes: fn (mut states &StateTracker) {
            states.brackets << "{"
        }
        categories: [.parenthesis, .open_paren]
    }
    ")": TokenEntry{
        type_: .close_paren
        state_changes: fn (mut states &StateTracker) {
            if states.brackets.len == 0 {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_2(")")
            } else if states.brackets.last() != "(" {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_0(")", states.brackets.last())
            }
            states.brackets.delete_last()
        }
        categories: [.parenthesis, .close_paren]
    }
    "]": TokenEntry{
        type_: .close_square_paren
        state_changes: fn (mut states &StateTracker) {
            if states.brackets.len == 0 {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_2("]")
            } else if states.brackets.last() != "[" {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_0("]", states.brackets.last())
            }
            states.brackets.delete_last()
        }
        categories: [.parenthesis, .close_paren]
    }
    "}": TokenEntry{
        type_: .close_curly_paren
        state_changes: fn (mut states &StateTracker) {
            if states.brackets.len == 0 {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_2("}")
            } else if states.brackets.last() != "{" {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_0("}", states.brackets.last())
            }
            states.brackets.delete_last()
        }
        categories: [.parenthesis, .close_paren]
    }
    ".": TokenEntry{
        type_: .dot_opr
        next_prohibited: r"[^\.]"
        categories: [.operator]
    }
    "hoi": TokenEntry{
        type_: .flag
        match_whole: true
        next_prohibited: r"\s"
    }
    "pub": TokenEntry{
        type_: .flag
        match_whole: true
        next_prohibited: r"\s"
    }
    "priv": TokenEntry{
        type_: .flag
        match_whole: true
        next_prohibited: r"\s"
    }
    "prot": TokenEntry{
        type_: .flag
        match_whole: true
        next_prohibited: r"\s"
    }
    "const": TokenEntry{
        type_: .flag
        match_whole: true
        next_prohibited: r"\s"
    }
    "true": TokenEntry{
        type_: .literal_misc
        match_whole: true
        next_prohibited: r"\s"
        categories: [.literal]
    }
    "false": TokenEntry{
        type_: .literal_misc
        match_whole: true
        next_prohibited: r"\s"
        categories: [.literal]
    }
    "null": TokenEntry{
        type_: .literal_misc
        match_whole: true
        next_prohibited: r"\s"
        categories: [.literal]
    }
    "inf": TokenEntry{
        type_: .literal_misc
        match_whole: true
        next_prohibited: r"\s"
        categories: [.literal]
    }
    "undef": TokenEntry{
        type_: .literal_misc
        match_whole: true
        next_prohibited: r"\s"
        categories: [.literal]
    }
    ";": TokenEntry{
        type_: .statement_end
        state_changes: fn(states &StateTracker) {
            if states.brackets.len != 0 && states.brackets.last() in ["(", "["] {
                error_pos(states.position.filename, states.position.line, states.position.column)
                error_2_0_1(states.brackets.last())
            }
        }
    }
    ",": TokenEntry{
        type_: .comma
    }
    ":": TokenEntry{
        type_: .colon
    }
    "": TokenEntry{
        type_: .literal_number
        prohibited: r"\D"
        next_prohibited: r"\D"
        categories: [.literal]
    }
    " ": TokenEntry{
        type_: .variable
        prohibited: r"\W"
        next_prohibited: r"[\W\s]"
    }
}

/* === PARSER === */
struct Element {
    line int
    column int
}
struct Comment {
    Element
    content string
}
struct Call {
    Element
    called ElementGroup
    args []ElementGroup
    // kwargs map[string]ElementGroup
}

enum UnaryOprType { // ++ -- + - ! \~
    increment
    decrement
    plussign
    minussign
    not
    bit_complement
}
const UnaryOprMap = {
    "++": UnaryOprType.increment
    "--": UnaryOprType.decrement
    "+": UnaryOprType.plussign
    "-": UnaryOprType.minussign
    "!": UnaryOprType.not
    "\~": UnaryOprType.bit_complement
}
enum BinaryOprType {
    logarithm
    root
    dotmult
    astmult
    crossmult
    div
    floordiv
    ceildiv
    rounddiv
    fractdiv
    floorfractdiv
    ceilfractdiv
    roundfractdiv
    modulo
    plus
    minus
    plusminus
    minusplus
    bit_lshift
    bit_rshift
    bit_0rshift
    and
    or_
    xor
    gt
    lt
    gteq
    lteq
    eq
    noteq
    istype
    isnttype
    is_
    isnt
    iseq
    isntnoteq
    bit_and
    bit_or
    bit_xor
    concat
    swap
}
const BinaryOprMap = {
    "lg": BinaryOprMap.logarithm
    "rt": BinaryOprMap.root
    "·": BinaryOprMap.dotmult
    "*": BinaryOprMap.astmult
    "×": BinaryOprMap.crossmult
    "÷": BinaryOprMap.div
    "÷f": BinaryOprMap.floordiv
    "÷c": BinaryOprMap.ceildiv
    "÷~": BinaryOprMap.rounddiv
    "/": BinaryOprMap.fractdiv
    "/c": BinaryOprMap.floorfractdiv
    "/f": BinaryOprMap.ceilfractdiv
    "/~": BinaryOprMap.roundfractdiv
    "%": BinaryOprMap.modulo
    "+": BinaryOprMap.plus
    "-": BinaryOprMap.minus
    "+-": BinaryOprMap.plusminus
    "-+": BinaryOprMap.minusplus
    "±": BinaryOprMap.plusminus
    "∓": BinaryOprMap.minusplus
    "\<<": BinaryOprMap.bit_lshift
    "\>>": BinaryOprMap.bit_rshift
    "\>>>": BinaryOprMap.bit_0rshift
    "&&": BinaryOprMap.and
    "||": BinaryOprMap.or_
    "^^": BinaryOprMap.xor
    ">": BinaryOprMap.gt
    "<": BinaryOprMap.lt
    "≥": BinaryOprMap.gteq
    "≤": BinaryOprMap.lteq
    "==": BinaryOprMap.eq
    "!=": BinaryOprMap.noteq
    "istype": BinaryOprMap.istype
    "isnttype": BinaryOprMap.isnttype
    "is": BinaryOprMap.is_
    "isnt": BinaryOprMap.isnt
    "===": BinaryOprMap.iseq
    "!==": BinaryOprMap.isntnoteq
    "\&": BinaryOprMap.bit_and
    "\|": BinaryOprMap.bit_or
    "\^": BinaryOprMap.bit_xor
    "..": BinaryOprMap.concat
    "><": BinaryOprMap.swap
}
struct UnaryOpr {
    Element
    type_ UnaryOprType
    operand ElementGroup
}
struct BinaryOpr {
    Element
    type_ BinaryOprType
    operand1 ElementGroup
    operand2 ElementGroup
}
struct TernaryOpr { // ?:
    Element
    condition1 ElementGroup
    branch1 ElementGroup
    branch2 ElementGroup
}

struct Literal {
    Element
    type_ Variable
    content string
}
struct LiteralFunction {
    Literal
    type_ Variable = Variable{name: "func"}
    content []ElementGroup
}

struct Variable {
    Element
    name string
}
struct VariableAttribute {
    Element
    name string
    parent ElementGroup
}

struct NullElement {
    Element
}

struct Statement {
    content []Token
}

type ElementGroup = Element
                  | Comment
                  | Call
                  | Literal | LiteralFunction
                  | Variable | VariableAttribute
                  | Token | NullElement

