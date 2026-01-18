use clap::{Parser};
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    cmd: String,
    path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    println!("IT WORKS! cmd: {:?} path: {:?}", args.cmd, args.path);

    for line in content.lines() {
        println!("{}", line);
    }
    Ok(())
}
