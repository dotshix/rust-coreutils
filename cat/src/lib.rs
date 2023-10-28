use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(name = "cat")]
#[command(author = "Dario S. github.com/dotshix")]
#[command(version = "0.1.0")]
#[command(about = "Rust cat implementation", long_about = None)]
pub struct Config {
    files: Vec<String>,

    /// Number the non-blank output lines, starting at 1.
    #[arg(short, conflicts_with= "n")]
    b: bool,

    /// Number the output lines, starting at 1.
    #[arg(short, conflicts_with = "b")]
    n: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let args = Config::parse();

    Ok(args)
}

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(config);
    for filename in config.files {
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}
