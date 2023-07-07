// use crate::loader::load;
// use crate::parser::parse;
use clap::Parser;
use std::collections::HashMap;

pub mod loader;
pub mod parser;
pub mod link;

use crate::link::Link;
use crate::parser::Span;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

type LinkIndex<'a> = HashMap<String, Vec<Link<'a>>>;

fn load_links(mut link_idx: LinkIndex, filename: String, links: Vec<Link>) ->  () {
    link_idx.insert(filename, links.clone());
    ()
}

fn handle_contents(link_idx: LinkIndex, filename: String, content: String) {
    let input = Span::new(&content);

    parser::parse(input)
          .map(|(_rest, links)| load_links(link_idx, filename, links));
}
fn main() {
    let args = Args::parse();
    let linkidx: HashMap<String, Vec<Link>> = HashMap::new();

    let res = loader::load(args.file.clone());
    res.map(|contents| handle_contents(linkidx, args.file.clone(), contents));

    // loader::load()
    // read a file
    // parse a file
    // load multiple files
    // build a graph
    // expose API to the graph
}
