pub use lexer::*;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Path to file to run")]
    path: std::path::PathBuf,
     #[arg(short='c', long, default_value_t = false, help = "Disable colored output")]
    no_colors: bool,
}

fn tokenize(text: &str) -> Result<Vec<Token>, PError> {
    Tokenizer::new(text).collect()
}

fn main() {
    let args = Cli::parse();
    let text = {
        std::fs::read_to_string(args.path.clone()).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
    };
    let path = args.path.to_str().unwrap();
    let colors = !args.no_colors;

    println!("INPUT:\t{}\n", text);
    let tokens = tokenize(&text).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });
    println!("TOKENS:\t{}\n", TokenVec(&tokens));

    let mut iter = tokens.iter();
    let node = parser::parse(&mut iter).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });

    println!("TREEs:\t{}\n", node);
    

    println!("EXECUTING:");
    let value = interpreter::interpret(node).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });

    println!("\nOUTPUT:\t{}\n", value);
}

