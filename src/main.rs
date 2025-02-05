use anyhow::{Context, Result};
use clap::Parser;
use gray_matter::Pod;
use gray_matter::{engine::YAML, Matter};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

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

fn main() -> Result<()> {
    let args = Args::parse();
    let matter = Matter::<YAML>::new();

    let mut existing_urls = Vec::new();
    for entry in WalkDir::new(&args.target)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Some(front_matter) = matter.parse(&content).data {
                if let Pod::Hash(hash_map) = front_matter {
                    if let Some(url) = hash_map.get("source_url").and_then(|v| v.as_string().ok()) {
                        existing_urls.push(url.to_string());
                    }
                }
            };
        }
    }

    for entry in WalkDir::new(&args.source)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        let content = fs::read_to_string(entry.path())
            .with_context(|| format!("Failed to read {}", entry.path().display()))?;

        if let Some(front_matter) = matter.parse(&content).data {
            if let Ok(url) = front_matter["source_url"].as_string() {
                if !existing_urls.contains(&url.to_string()) {
                    let rel_path = entry.path().strip_prefix(&args.source)?;
                    let target_path = args.target.join(rel_path);

                    if args.dry_run {
                        println!(
                            "Would copy {} to {}",
                            entry.path().display(),
                            target_path.display()
                        );
                    } else {
                        if let Some(parent) = target_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::copy(entry.path(), &target_path).with_context(|| {
                            format!("Failed to copy to {}", target_path.display())
                        })?;
                        println!(
                            "Copied {} to {}",
                            entry.path().display(),
                            target_path.display()
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
