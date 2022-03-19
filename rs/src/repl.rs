use std::io;
use std::io::Write;
use text_io::read;
use crate::{lex, parse_token_list};
use crate::instructor::gen_instructions_from_block;
use crate::interpreter::interpret_expr;
use crate::objects::typeobj::TypeObj;
use crate::objects::variable::Variable;
use crate::objects::varstack::Varstack;

pub fn repl(debug_info: bool) {
    let filename = "[stdin]".to_string();
    let mut typelist = Varstack::<TypeObj>::default_type();
    let mut varlist = Varstack::<Variable>::default_variable();
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let input: String = read!("{}\n");
        // TODO support for multiline

        if input == ";exit".to_string() {break;}
        let instructions = gen_instructions_from_block(
            parse_token_list(lex(input, &filename).unwrap(), &filename),
            &mut typelist
        );
        for instr in instructions {
            let result = interpret_expr(instr, &mut varlist);
            if result != Variable::Null {println!("{}", result)}
        }
    }
}