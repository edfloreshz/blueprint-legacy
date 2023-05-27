mod args;
mod config;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Provided file -> {}!", args.file)
}
