// use crate::loader::load;
// use crate::parser::parse;
use clap::Parser;

pub mod loader;
pub mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    // read a file
    // parse a file
    // load multiple files
    // build a graph
    // expose API to the graph
    ()
}
