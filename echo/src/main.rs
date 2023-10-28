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

    #[arg(required = false, allow_hyphen_values = true)]
    /// Input text
    text: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut found_double_dash = false;

    // Check if the arguments contain "--" and it is not the first argument (which is the program name)
    if (args.len() == 2 && args[1] == "--") || (args.len() == 3 && args[1] == "-n" && args[2] == "--") {
        found_double_dash = true;
    }

    // Parse the arguments with clap
    let cli = CLI::parse();

    // Determine the line ending and output text
    let ending = if cli.n { "" } else { "\n" };
    let output = if cli.text.len() == 1 {
        cli.text[0].clone()
    } else {
        cli.text.join(" ")
    };

    // Print the output
    if found_double_dash {
        print!("--{}", ending);
    } else {
        print!("{}{}", output, ending);
    }
}
