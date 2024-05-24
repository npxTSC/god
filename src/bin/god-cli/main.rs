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

    /// path for a custom chromium binary
    #[arg(short, long)]
    chromium: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let state = god::get_datafile(args.dotfile.as_deref());
    let state = god::read_datafile(&state);

    let _browser = god::new_browser(&state).unwrap();
}
