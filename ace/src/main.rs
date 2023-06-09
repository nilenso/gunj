use clap::Parser;
use crate::loader::load;
use crate::parser::parse;

pub mod loader;
pub mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String
}

fn main() {
    let args = Args::parse();

    match load(args.file) {
        Ok(contents) => {
            let parsed = parse(contents);
            println!("Parsed: {}", parsed);
        },
        Err(err) => ()
    };

    // read a file
    // parse a file
    // load multiple files
    // build a graph
    // expose API to the graph
    ()
}


