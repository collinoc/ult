mod lexer;
use lexer::lexer::Lexer;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1);

    let mut lexer = Lexer::new(filepath);

    let start = std::time::Instant::now();

    lexer.lex()?;

    let end = std::time::Instant::now();

    for token in lexer.tokens() {
        println!("{:?}", token);
    }

    println!("Done in {:?}", (end - start));

    Ok(())
}
