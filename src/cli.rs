use std::sync::OnceLock;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "ZWinSet")]
#[command(author, version, about = "App with CLI support")]
pub struct Cli {
    #[arg(long)]
    debug: bool,

    #[arg(short, long)]
    pub interactive_mode: Option<String>,

    #[arg(short, long)]
    config: Option<String>,

    #[arg(last = true)]
    extra: Vec<String>,
}
pub static CLI: OnceLock<Cli> = OnceLock::new();
