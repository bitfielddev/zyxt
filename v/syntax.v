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

enum OprType {
    increment
    decrement
    plussign
    minussign
    not
    bit_complement
    logarithm
    root
    power
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
    isnteq
    bit_and
    bit_or
    bit_xor
    concat
    swap
    null
}
const UnaryOprMap = {
    "++": OprType.increment
    "--": OprType.decrement
    "+": OprType.plussign
    "-": OprType.minussign
    "!": OprType.not
    "\\~": OprType.bit_complement
}
const BinaryOprMap = {
    "lg": OprType.logarithm
    "rt": OprType.root
    "^": OprType.power
    "·": OprType.dotmult
    "*": OprType.astmult
    "×": OprType.crossmult
    "÷": OprType.div
    "÷f": OprType.floordiv
    "÷c": OprType.ceildiv
    "÷~": OprType.rounddiv
    "/": OprType.fractdiv
    "/c": OprType.floorfractdiv
    "/f": OprType.ceilfractdiv
    "/~": OprType.roundfractdiv
    "%": OprType.modulo
    "+": OprType.plus
    "-": OprType.minus
    "+-": OprType.plusminus
    "-+": OprType.minusplus
    "±": OprType.plusminus
    "∓": OprType.minusplus
    "\\<<": OprType.bit_lshift
    "\\>>": OprType.bit_rshift
    "\\>>>": OprType.bit_0rshift
    "&&": OprType.and
    "||": OprType.or_
    "^^": OprType.xor
    ">": OprType.gt
    "<": OprType.lt
    "≥": OprType.gteq
    "≤": OprType.lteq
    "==": OprType.eq
    "!=": OprType.noteq
    "istype": OprType.istype
    "isnttype": OprType.isnttype
    "is": OprType.is_
    "isnt": OprType.isnt
    "===": OprType.iseq
    "!==": OprType.isnteq
    "\\&": OprType.bit_and
    "\\|": OprType.bit_or
    "\\^": OprType.bit_xor
    "..": OprType.concat
    "><": OprType.swap
}
const OrderMap = {
    OprType.increment: 2
    OprType.decrement: 2
    OprType.plussign: 2
    OprType.minussign: 2
    OprType.not: 2
    OprType.bit_complement: 2
    OprType.logarithm: 4
    OprType.root: 4
    OprType.power: 3
    OprType.dotmult: 5
    OprType.astmult: 6
    OprType.crossmult: 7
    OprType.div: 7
    OprType.floordiv: 7
    OprType.ceildiv: 7
    OprType.rounddiv: 7
    OprType.fractdiv: 6
    OprType.floorfractdiv: 6
    OprType.ceilfractdiv: 6
    OprType.roundfractdiv: 6
    OprType.modulo: 6
    OprType.plus: 8
    OprType.minus: 8
    OprType.plusminus: 8
    OprType.minusplus: 8
    OprType.bit_lshift: 9
    OprType.bit_rshift: 9
    OprType.bit_0rshift: 9
    OprType.and: 14
    OprType.or_: 16
    OprType.xor: 15
    OprType.gt: 10
    OprType.lt: 10
    OprType.gteq: 10
    OprType.lteq: 10
    OprType.eq: 10
    OprType.noteq: 10
    OprType.istype: 10
    OprType.isnttype: 10
    OprType.is_: 10
    OprType.isnt: 10
    OprType.iseq: 10
    OprType.isntnoteq: 10
    OprType.bit_and: 11
    OprType.bit_or: 13
    OprType.bit_xor: 12
    OprType.concat: 17
    OprType.swap: 19
}
struct UnaryOpr {
    Element
    type_ OprType
    operand ElementGroup
}
struct BinaryOpr {
    Element
    type_ OprType
    operand1 ElementGroup
    operand2 ElementGroup
}
struct TernaryOpr { // ?:
    Element
    condition1 ElementGroup
    branch1 ElementGroup
    branch2 ElementGroup
}
struct AssignmentOpr {
    Element
    variable Variable
    content ElementGroup
    flags []Flag
    type_ Variable
    operation OprType = .null
}
const FlagMap = {
    "hoi": Flag.hoi
    "pub": Flag.pub_
    "priv": Flag.priv
    "prot": Flag.prot
    "const": Flag.const_
}
enum Flag {
    hoi
    pub_
    priv
    prot
    const_
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

