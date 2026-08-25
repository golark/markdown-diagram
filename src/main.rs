use clap::Parser;
use std::fs;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    markdown: String,
}

#[derive(Debug)]
struct Node {
    text: String,
    arrows: Vec<Box<Node>>
}

impl Node {
    fn new(text: String) -> Self {
        Node {
            text,
            arrows: Vec::new(), 
        }
    }
}


fn main() {
    let args = Args::parse();
    eprintln!("reading file...");

    let content = match fs::read_to_string(&args.markdown) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", &args.markdown, e);
            std::process::exit(1);
        }
    };
    eprintln!("file read, {} bytes", content.len());

    let mut nodes = Node::new("x".to_string());
    println!("{:?}", nodes);

    //let re = Regex::new(r"(?P<prefix>[│├└─\s]*)(?P<arrow>[>v^])?\s*(?P<text>\[.*?\])").unwrap();
    //eprintln!("regex compiled");
}
