use std::io::Read;

use clap::{Args, Parser, Subcommand};
use tabula::{Caesar, Encrypt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file using a specified algorithm
    Encrypt(EncryptArgs),
    /// Decrypt a file using a specified algorithm
    Decrypt(EncryptArgs),
    /// Run cryptanalysis toolset on a file
    Analyze(AnalyzeArgs),
}

#[derive(Args)]
struct EncryptArgs {
    #[command(subcommand)]
    algorithm: Algorithm,
    #[arg(global=true, short, long)] 
    #[arg(help="The input file to encrypt/decrypt; if not provided, input will be read from STDIN")]
    input: Option<String>,
}

#[derive(Subcommand)]
enum Algorithm {
    /// Use the Caesar cipher with the specified shift
    Caesar { shift: i8 },
    /// Use a multiplicative cipher with the specified multiplier
    Multiplicative { multiplier: i8 },
}

#[derive(Args)]
struct AnalyzeArgs {
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encrypt(args) => {
            match args.algorithm {
                Algorithm::Caesar { shift } => {
                    let cipher = Caesar::new(shift);
                    let input = args.input.map_or_else(
                        || {
                            let mut buffer = String::new();
                            std::io::stdin().read_to_string(&mut buffer).unwrap();
                            buffer
                        },
                        |filename| {
                            std::fs::read_to_string(filename).unwrap()
                        }
                    );
                    let output = input.encrypt_with(cipher);
                    println!("{}", output);
                }
                _ => todo!(),
            }
        },
        _ => todo!(),
    }
}
