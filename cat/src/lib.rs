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

pub fn read_lines(file: Box<dyn BufRead>, config: &Config) -> MyResult<()> {
    let mut line_number = 1;

    for line in file.lines() {
        let line = line?;
        if config.n {
            println!("{:6}\t{}", line_number, line);
            line_number += 1;
        } else if config.b && !line.is_empty() {
            println!("{:6}\t{}", line_number, line);
            line_number += 1;
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open_file(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                read_lines(file, &config)?;
            }
        }
    }
    Ok(())
}
