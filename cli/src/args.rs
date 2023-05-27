use crate::config::{ABOUT, AUTHOR, VERSION};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
pub struct Args {
    #[arg(short = 'f', long = "file")]
    pub(crate) file: String,
}
