use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echo")]
#[command(author = "Dario S. github.com/dotshix")]
#[command(version = "0.1.0")]
#[command(about = "Rust echo implementation", long_about = None)]
struct CLI {
    #[arg(short)]
    /// Omits newline
    n: bool,

    #[arg(required = true)]
    /// Input text
    text: Vec<String>,
}

fn main() {
    let cli = CLI::parse();
    let ending = if cli.n { "" }  else { "\n" };

    let output = if cli.text.len() == 1 { cli.text[0].clone() } else { cli.text.join(" ") };

    print!("{}{}", output, ending);
}
