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
struct ArenaNode {
    text: String,
    arrows: Vec<(String, usize)>,
}

struct Arena {
    nodes: Vec<ArenaNode>,
}

impl Arena {
    fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    fn push(&mut self, text: String) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(ArenaNode { text, arrows: Vec::new() });
        idx
    }

    fn add_arrow(&mut self, parent: usize, direction: String, child: usize) {
        self.nodes[parent].arrows.push((direction, child));
    }

    fn display(&self, idx: usize, prefix: &str) {
        let node = &self.nodes[idx];
        if idx != 0 {
            print!("{}{}", prefix, node.text);
        }
        if idx == 0 {
            println!();
        }
        for (dir, child_idx) in &node.arrows {
            if dir == "v" {
                println!();
            }
            print!("{}{} ", prefix, dir);
            self.display(*child_idx, &format!("{}  ", prefix));
        }
    }

    fn compose_svg(&self, idx: usize, x: f64, y: f64) -> (String, f64) {
        let node = &self.nodes[idx];
        let mut svg = String::new();
        let text_w = (node.text.len() as f64) * 10.0 + 40.0;
        let text_h = 44.0;

        if idx == 0 {
            let mut child_y = y;
            for (_dir, child_idx) in &node.arrows {
                let (child_svg, last_y) = self.compose_svg(*child_idx, x, child_y);
                svg.push_str(&child_svg);
                child_y = last_y + 40.0;
            }
            return (svg, child_y - 40.0);
        }

        let box_x = x;
        let box_y = y;

        // Hand-sketched sketchy box path with slight wobbles on each edge
        svg.push_str(&format!(
            "<rect x=\"{:.0}\" y=\"{:.0}\" width=\"{:.0}\" height=\"{:.0}\" rx=\"14\" ry=\"14\" \
             fill=\"white\" stroke=\"#1A1A1A\" stroke-width=\"2.2\" stroke-linejoin=\"round\" />\n\
             <text x=\"{:.0}\" y=\"{:.0}\" font-size=\"16\" font-weight=\"bold\" fill=\"#1A1A1A\">{}</text>\n",
            box_x, box_y, text_w, text_h,
            box_x + 14.0, box_y + 28.0, node.text
        ));

        let mut child_y = y + text_h + 50.0;
        for (dir, child_idx) in &node.arrows {
            let (child_x, child_start_y) = match dir.as_str() {
                ">" | "->" => (box_x + text_w + 50.0, box_y),
                "^" => (box_x, y - 60.0),
                _ => (box_x + 10.0, child_y),
            };
            let start_y = if matches!(dir.as_str(), ">" | "->") { child_start_y } else { child_start_y.max(y + text_h + 20.0) };
            let (child_svg, last_y) = self.compose_svg(*child_idx, child_x, start_y);

            let end_x = child_x;
            let end_y = start_y;

            let path = match dir.as_str() {
                ">" | "->" => {
                    let sx = box_x + text_w;
                    let sy = box_y + text_h * 0.5;
                    let ex = end_x;
                    let ey = sy;
                    format!(
                        "<path d=\"M {:.0},{:.0} Q {:.0},{:.0} {:.0},{:.0}\" fill=\"none\" stroke=\"#1A1A1A\" stroke-width=\"2.2\" stroke-linecap=\"round\" marker-end=\"url(#arrow)\" />\n",
                        sx, sy, sx + (ex - sx) * 0.5, sy - 5.0, ex, ey
                    )
                }
                "^" => {
                    let sx = box_x + text_w * 0.5;
                    let sy = box_y;
                    let ex = end_x + text_w * 0.5;
                    let ey = end_y + text_h;
                    format!(
                        "<path d=\"M {:.0},{:.0} Q {:.0},{:.0} {:.0},{:.0}\" fill=\"none\" stroke=\"#1A1A1A\" stroke-width=\"2.2\" stroke-linecap=\"round\" marker-end=\"url(#arrow)\" />\n",
                        sx, sy, sx - 10.0, sy - (sy - ey) * 0.5, ex, ey
                    )
                }
                _ => {
                    let sx = box_x + text_w * 0.5;
                    let sy = box_y + text_h;
                    let ex = end_x + text_w * 0.5;
                    let ey = end_y;
                    format!(
                        "<path d=\"M {:.0},{:.0} Q {:.0},{:.0} {:.0},{:.0}\" fill=\"none\" stroke=\"#1A1A1A\" stroke-width=\"2.2\" stroke-linecap=\"round\" marker-end=\"url(#arrow)\" />\n",
                        sx, sy, sx + 8.0, sy + (ey - sy) * 0.5, ex, ey
                    )
                }
            };
            svg.push_str(&path);
            svg.push_str(&child_svg);
            child_y = last_y + 40.0;
        }

        (svg, child_y - 40.0)
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

    let re = Regex::new(r"(?P<prefix>[│├└─\s]*)(?P<arrow>->|[>v^])?\s*(?P<text>\[.*?\])").unwrap();

    let mut arena = Arena::new();
    let root_idx = arena.push("Root".to_string());
    let mut stack: Vec<usize> = Vec::new();

    for captures in re.captures_iter(&content) {
        let prefix = captures.name("prefix").map_or("", |m| m.as_str());
        let arrow = captures.name("arrow").map_or("", |m| m.as_str()).to_string();
        let text = captures.name("text")
            .map_or("", |m| m.as_str())
            .trim_start_matches('[')
            .trim_end_matches(']')
            .trim()
            .to_string();

        let has_connector = prefix.contains('\u{251C}') || prefix.contains('\u{2514}');
        let has_newline = prefix.contains('\n');

        let depth = if has_connector {
            let chars_before: usize = prefix.chars().take_while(|c| *c == ' ' || *c == '\u{2502}').count();
            chars_before / 2 + 1
        } else if has_newline {
            0
        } else {
            stack.len()
        };

        println!("Prefix: '{}', Arrow: '{}', Text: '{}', Depth: {}", prefix, arrow, text, depth);

        let child_idx = arena.push(text);
        stack.truncate(depth);

        let parent_idx = if stack.is_empty() { root_idx } else { *stack.last().unwrap() };
        arena.add_arrow(parent_idx, arrow, child_idx);
        stack.push(child_idx);
    }

    arena.display(root_idx, "");
    println!();

    let (svg, height) = arena.compose_svg(root_idx, 20.0, 20.0);
    fs::write("diagram.svg", format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"{:.0}\">\n\
         <defs>\n\
           <marker id=\"arrow\" markerWidth=\"10\" markerHeight=\"10\" refX=\"8\" refY=\"4\" orient=\"auto\">\n\
             <polygon points=\"0 1, 8 4, 0 7\" fill=\"#2C2C2C\" />\n\
           </marker>\n\
         </defs>\n\
         <rect width=\"100%\" height=\"100%\" fill=\"#FDFBF7\" />\n\
         {}</svg>",
        height + 40.0, svg
    )).unwrap();

    eprintln!("diagram.svg written");
}
