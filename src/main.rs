use anyhow::{Context, Result};
use clap::Parser;
use grrs::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let _ = find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
