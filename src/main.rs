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
    arrows: Vec<Arrow>
}

#[derive(Debug)]
struct Arrow {
    direction: String,
    node: Box<Node>,
}

impl Node {
    fn new(text: String) -> Self {
        Node {
            text,
            arrows: Vec::new(), 
        }
    }

    fn display(&self, prefix: &str) {
        print!("{}{}", prefix, self.text);

        if self.text == "Root" {
            println!();
        }
        for arrow in &self.arrows {

            if arrow.direction == "v" {
                println!();
            }
            print!("{}{}", prefix, arrow.direction);
            arrow.node.display(&format!("{}  ", prefix));
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

    // create a pointer to the root node
    let mut root = Node::new("Root".to_string()); 

    let re = Regex::new(r"(?P<prefix>[│├└─\s]*)(?P<arrow>->|[>v^])?\s*(?P<text>\[.*?\])").unwrap();

    for captures in re.captures_iter(&content) {
        let prefix = captures.name("prefix").map_or("", |m| m.as_str());
        let arrow = captures.name("arrow").map_or("", |m| m.as_str());
        let text = captures.name("text").map_or("", |m| m.as_str().trim_start_matches('[').trim_end_matches(']').trim());

        // if node is empty, create a new node and add it to the nodes vector
        if root.arrows.is_empty() {
            root.arrows.push(Arrow {
                direction: arrow.to_string(),
                node: Box::new(Node::new(text.to_string())),
            });
        } else {
            // if node is not empty, add the new node as a child of the last node
            let last_node = root.arrows.last_mut().unwrap();

            let new_node = Node::new(text.to_string());
            last_node.node.arrows.push(Arrow {
                direction: arrow.to_string(),
                node: Box::new(new_node),
            });
        }

        println!("Prefix: '{}', Arrow: '{}', Text: '{}'", prefix, arrow, text);
    }

    root.display("");
}
