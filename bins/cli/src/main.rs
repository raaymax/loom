use interpreter::VType;
pub use lexer::*;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
     #[arg(short='c', long, default_value_t = false, help = "Disable colored output")]
    no_colors: bool,
     #[arg(short='v', long, default_value_t = false, help = "Enable verbose output")]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// run a file
    Run {
        #[arg(help = "Path to file to run")]
        path: std::path::PathBuf,
    },
    Run2 {
        #[arg(help = "Path to file to run")]
        path: std::path::PathBuf,
    },
}

fn tokenize(text: &str) -> Result<Vec<Token>, PError> {
    Tokenizer::new(text).collect()
}

fn main() {
    let args = Cli::parse();
    if args.command.is_none() {
        eprintln!("No command provided. Use --help for more information.");
        std::process::exit(1);
    }
    let cmd = args.command.unwrap();
    match cmd {
        Commands::Run { path } => {
            let path_str = path.to_str().unwrap();
            let text = {
                std::fs::read_to_string(path.clone()).unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    std::process::exit(1);
                })
            };
            if !path.exists() {
                eprintln!("File not found: {}", path_str);
                std::process::exit(1);
            }
            let colors = !args.no_colors;

            if args.verbose {
                println!("INPUT:\t{}\n", text);
            }
            let tokens = tokenize(&text).unwrap_or_else(|e| {
                eprintln!("{}", e.format_error(&text, path_str, colors));
                std::process::exit(1);
            });
            if args.verbose {
                println!("TOKENS:\t{}\n", TokenVec(&tokens));
            }

            let mut iter = tokens.iter();
            let node = parser::parse(&mut iter).unwrap_or_else(|e| {
                eprintln!("{}", e.format_error(&text, path_str, colors));
                std::process::exit(1);
            });

            if args.verbose {
                println!("TREEs:\t{}\n", node);
            }

            if args.verbose {
                println!("EXECUTING:");
            }
            let value = interpreter::interpret(node).unwrap_or_else(|e| {
                eprintln!("{}", e.format_error(&text, path_str, colors));
                std::process::exit(1);
            });

            if let VType::Undefined = value {
                println!("\nExited with code: 0");
                std::process::exit(0);
            }
            if let VType::Number(n)= value {
                println!("\nExited with code: {}", n);
                std::process::exit(n);
            }
            println!("\nExited with code: {}", value);
            std::process::exit(0);
        },
        Commands::Run2 { path } => {
            let path_str = path.to_str().unwrap();
            let text = {
                std::fs::read_to_string(path.clone()).unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    std::process::exit(1);
                })
            };
            if !path.exists() {
                eprintln!("File not found: {}", path_str);
                std::process::exit(1);
            }
            let colors = !args.no_colors;

            if args.verbose {
                println!("INPUT:\t{}\n", text);
            }
            let tokens = tokenize(&text).unwrap_or_else(|e| {
                eprintln!("{}", e.format_error(&text, path_str, colors));
                std::process::exit(1);
            });
            if args.verbose {
                println!("TOKENS:\t{}\n", TokenVec(&tokens));
            }

            let mut iter = tokens.iter();
            let node = parser::parse(&mut iter).unwrap_or_else(|e| {
                eprintln!("{}", e.format_error(&text, path_str, colors));
                std::process::exit(1);
            });

            if args.verbose {
                println!("TREEs:\t{}\n", node);
            }

            if args.verbose {
                println!("EXECUTING:");
            }
            let value = interpreter2::interpret(node).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            std::process::exit(0);
        },
    }
}

