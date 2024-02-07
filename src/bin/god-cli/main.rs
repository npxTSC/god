use std::path::PathBuf;

use clap::Parser;
use god;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(author, version, about)]
/// `god` is a CLI tool for accumulating online aliases...
pub struct Args {
    /// the username to investigate
    user: Option<String>,

    /// the persistent data file (defaults to ~/.god-data)
    #[arg(short, long)]
    dotfile: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let _datafile = god::get_datafile(args.dotfile.as_deref());
}
