mod lex; mod parse; mod codegen; /* mod eval; */

use lex::lexer::Lexer;
use parse::parser::Parser;
use codegen::compiler::Compiler;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1);

    let mut lexer = Lexer::new(filepath);

    let start = std::time::Instant::now();  // Begin program

    lexer.lex()?;

    let mut parser = Parser::new(lexer.tokens());

    let ast = parser.parse()?;

    let compiler = Compiler::new(&ast);

    compiler.compile();
    
    let end = std::time::Instant::now();    // End program

    for node in ast.program() {
        println!("{:?}", node);
    }

    compiler.dump();
    
    println!("Done in {:?}", (end - start));

    Ok(())
}
