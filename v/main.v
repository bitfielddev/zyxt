import os
const version = "0.0.0"

fn compile(input string) []Token {
	return lex(input)
}

fn main() {
	cmd := os.args[1] or {""}
	match cmd {
		'version' {
			println("Zyxt version" + version)
		}
		'run' {
			filename := os.args[2] or {error_0_1()}
			mut content := os.read_file(filename) or {error_1_1()}
			println(content)
			println(compile(mut content))
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