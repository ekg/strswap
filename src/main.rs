use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Rewrite strings in the input
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to execute replacements
    #[arg(short, long)]
    input: String,
    /// Swaps to execute, with each pattern on a separate line, whitespace delimited
    #[arg(short, long)]
    swaps: String,
}

fn main() {
    let args = Args::parse();
    let input = getfile(&args.input);
    println!("{}", input);
    let swaps = getfile(&args.swaps);
    println!("{}", swaps);
}

fn getfile(p: &str) -> String {
    let path = Path::new(p);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    };
    s
}
