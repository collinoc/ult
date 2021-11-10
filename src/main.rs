mod lexer;
use lexer::lexer::Lexer;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let filepath = args.get(1);

    let mut lexer = Lexer::new(filepath);

    let start = std::time::Instant::now();

    match lexer.lex() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    let end = std::time::Instant::now();

    // for token in lexer.tokens() {
    //     println!("{:?}", token);
    // }

    println!("Done in {:?}", (end - start));
}
