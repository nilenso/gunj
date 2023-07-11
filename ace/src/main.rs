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

type LinkIndex<'a> = HashMap<&'a str, Vec<Link<'a>>>;

fn load_links<'a>(link_idx: &'a mut LinkIndex<'a>, filename: &'a str, links: Vec<Link<'a>>) -> () {
    link_idx.insert(filename, links);
    println!("{:?}", link_idx);
    ()
}

fn handle_contents<'a>(link_idx: &'a mut LinkIndex<'a>, filename: &'a str, content: &'a str) -> () {
    let input = Span::new(&content);

    parser::parse(input)
          .map(|(_rest, links)| load_links(link_idx, filename, links));
    ()
}
fn main() {
    let args = Args::parse();

    let res = loader::load(args.file.clone());
    res.map(|contents| {
        let mut linkidx: HashMap<&str, Vec<Link>> = HashMap::new();
        let filename = &args.file.clone();
        handle_contents(&mut linkidx, filename, &contents)
    });

    // loader::load()
    // read a file
    // parse a file
    // load multiple files
    // build a graph
    // expose API to the graph
}
