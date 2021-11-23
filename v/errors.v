[noreturn]
fn error_main(code string, msg string) {
    println(ansi['background_bright_red']+"Error $code"+ansi['reset']+ansi['bright_red']+" $msg"+ansi['reset'])
    exit(1)
}

fn error_pos(filename string, line int, column int) {
    print(ansi['background_bright_red']+"$filename:$line:$column "+ansi['reset'])
}

/* 0. Internal errors, have to do with the compiler-interpreter itself */
// V error
[noreturn]
fn error_0_0(stack string) {
    error_main("0.0", "Internal error: \n$stack")
}

// No file given
[noreturn]
fn error_0_1() {
    error_main("0.1", "No file given")
}

/* 1. File and I/O errors */
// File does not exist
[noreturn]
fn error_1_0(filename string) {
    error_main("1.0", "File `$filename` does not exist")
}

// file cannot be opened
[noreturn]
fn error_1_1(filename string) {
    error_main("1.1", "File `$filename` cannot be opened")
}

/* 2. Syntax errors */
// parentheses not closed properly
[noreturn]
fn error_2_0_0(paren1 string, paren2 string) {
    error_main("2.0.0", "Parentheses `$paren1` and `$paren2` not closed properly; try swapping them")
}
[noreturn]
fn error_2_0_1(paren string) {
    error_main("2.0.1", "Parenthesis `$paren` not closed")
}
[noreturn]
fn error_2_0_2(paren string) {
    error_main("2.0.2", "Parenthesis `$paren` not opened")
}

// unexpected ident
fn error_2_1(ident string) {
    error_main("2.1", "Unexpected ident `$ident`")
}

/* 3. Variable & attribute errors */
// Variable not defined
[noreturn]
fn error_3_0(varname string) {
    error_main("3.0", "Undefined variable `$varname`")
}