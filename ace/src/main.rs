use clap::Parser;
use crate::loader::load;

pub mod loader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String
}

fn main() {
    let args = Args::parse();

    println!("{}", match load(args.file) {
        Ok(str) => str,
        Err(err) => err.to_string()
    });

    // read a file
    // parse a file
    // load multiple files
    // build a graph
    // expose API to the graph
    ()
}


