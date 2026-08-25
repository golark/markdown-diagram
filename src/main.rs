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

    let mut nodes = Vec::new();
    println!("{:?}", nodes);

    let re = Regex::new(r"(?P<prefix>[│├└─\s]*)(?P<arrow>[>v^])?\s*(?P<text>\[.*?\])").unwrap();

    for captures in re.captures_iter(&content) {
        let prefix = captures.name("prefix").map_or("", |m| m.as_str());
        let arrow = captures.name("arrow").map_or("", |m| m.as_str());
        let text = captures.name("text").map_or("", |m| m.as_str());

        // if node is empty, create a new node and add it to the nodes vector
        if nodes.is_empty() {
            let new_node = Node::new(text.to_string());
            nodes.push(Box::new(new_node));
        } else {
            // if node is not empty, add the new node as a child of the last node
            let last_node = nodes.last_mut().unwrap();
            let new_node = Node::new(text.to_string());
            last_node.arrows.push(Box::new(new_node));
        }

        println!("Prefix: '{}', Arrow: '{}', Text: '{}'", prefix, arrow, text);
    }
}
