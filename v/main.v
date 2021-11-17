import os
import time
const version = "0.0.0"

fn compile(input string) []Token {
    start := time.now()
    println("Lexing")
    lexed := lex(input)
    after_lexed := time.now()
    println(lexed)

    println("Parsing")
    parsed := parse(lexed)
    after_parsed := time.now()
    println(parsed)

    println("Lexing time: ${(after_lexed - start).microseconds()}µs")
    println("Parsing time: ${(after_parsed - after_lexed).microseconds()}µs")
    println("Total time: ${(after_parsed - start).microseconds()}µs")

    out := []Token{}
    return out
}

fn main() {
    cmd := os.args[1] or {""}
    match cmd {
        'version' {
            println("Zyxt version" + version)
        }
        'run' {
            filename := os.args[2] or {error_0_1()}
            content := os.read_file(filename) or {error_1_1(filename)}
            println(compile(content))
        }
        'compile' {
            println("Coming soon!")
        }
        'interpret' {
            println("Coming soon!")
        }
        else {
            // print help page
        }
    }
}