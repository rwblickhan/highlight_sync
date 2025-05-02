use std::path::PathBuf;

use clap::{CommandFactory, Parser};

// #KeepArgsInSync
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Source directory containing Markdown files to sync
    #[arg(short, long)]
    pub source: PathBuf,

    /// Target directory to copy unique files into
    #[arg(short, long)]
    pub target: PathBuf,

    /// Dry run - show what would be copied without copying
    #[arg(short, long)]
    pub dry_run: bool,
}

fn main() -> std::io::Result<()> {
    // let out_dir = std::path::PathBuf::from("");

    let man = clap_mangen::Man::new(Args::command());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write("target/release/highlight_sync.1", buffer)?;

    Ok(())
}
