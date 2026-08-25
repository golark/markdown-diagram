use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input markdown file
    #[arg(short, long)]
    markdown: String,

}

fn main() {
    let args = Args::parse();

    let file = match File::open(&args.markdown) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", &args.markdown, e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                std::process::exit(1);
            }
        }
    }
}