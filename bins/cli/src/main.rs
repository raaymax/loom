pub use lexer::*;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(help = "Path to file to run")]
    path: std::path::PathBuf,
     #[arg(short='c', long, default_value_t = false, help = "Disable colored output")]
    no_colors: bool,
     #[arg(short='v', long, default_value_t = false, help = "Enable verbose output")]
    verbose: bool,
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

    if args.verbose {
        println!("INPUT:\t{}\n", text);
    }
    let tokens = tokenize(&text).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });
    if args.verbose {
        println!("TOKENS:\t{}\n", TokenVec(&tokens));
    }

    let mut iter = tokens.iter();
    let node = parser::parse(&mut iter).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });

    if args.verbose {
        println!("TREEs:\t{}\n", node);
    }


    if args.verbose {
        println!("EXECUTING:");
    }
    let value = interpreter::interpret(node).unwrap_or_else(|e| {
        eprintln!("{}", e.format_error(&text, path, colors));
        std::process::exit(1);
    });

    println!("\nExited with code: {}", value);
}

