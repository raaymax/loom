pub use lexer::*;

fn tokenize(text: &str) -> Result<Vec<Token>, PError> {
    Tokenizer::new(text).collect()
}

fn main() {
    let text = if let Some(path) = std::env::args().nth(1) {
        std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("Error: \n{}", e);
            std::process::exit(1);
        })
    } else {
        let name = std::env::args().next().unwrap();
        eprintln!("Usage: {} <filename>", name);
        std::process::exit(1);
    };

    println!("\nINPUT:\t{}", text);
    let tokens = tokenize(&text).unwrap_or_else(|e| {
        eprintln!("Error: \n{}", e.format_error(&text));
        std::process::exit(1);
    });
    println!("\nTOKENS:\t{}", TokenVec(&tokens));

    let mut iter = tokens.iter();
    let node = parser::parse(&mut iter).unwrap_or_else(|e| {
        eprintln!("Error: \n{}", e.format_error(&text));
        std::process::exit(1);
    });

    println!("\nTREEs:\t{}", node);
    
    let value = interpreter::interpret(node).unwrap_or_else(|e| {
        eprintln!("Error: \n{}", e);
        std::process::exit(1);
    });

    println!("\nOUTPUT:\t{}", value);
}

