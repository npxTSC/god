use std::path::PathBuf;

use clap::Parser;
use god::{self, Configs};

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(author, version, about)]
/// `god` is a CLI tool for accumulating online aliases...
pub struct Args {
    /// the username to investigate
    user: String,

    /// path for a custom chromium binary
    #[arg(short, long)]
    chromium: Option<PathBuf>,

    /// should the browser be visible?
    #[arg(short, long, action)]
    visible: bool,
}

fn main() {
    let args = Args::parse();

    let conf = Configs {
        chromium: args.chromium,
        headless: args.visible,
    };

    let _browser = god::start_scan(&conf, 1, &args.user).unwrap();
}
