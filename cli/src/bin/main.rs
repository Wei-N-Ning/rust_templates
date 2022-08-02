use clap::Parser;
use lib_cli::prelude::*; // TODO: replace it with your library

// reference
// https://github.com/clap-rs/clap

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}, {}", args.name, add(1, 2))
    }
}
