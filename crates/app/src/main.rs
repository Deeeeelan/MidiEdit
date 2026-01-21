use clap::{Parser};
use anyhow::{Context, Result};
use midiedit_cli;
use midiedit_core::{Cli, Commands, };


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Read(read_file) => {
            let content = std::fs::read_to_string(&read_file.file)
                .with_context(|| format!("could not read file `{}`", read_file.file.display()))?;

            println!("IT WORKS! path: {:?}", read_file.file);

            for line in content.lines() {
                println!("{}", line);
            }
        }
        Commands::Test(file_path) => {
            midiedit_cli::read_file(file_path.file.clone())?;
            
        }
        Commands::Transpose(args) => {
            midiedit_cli::transpose(args.file.clone(), args.amt, args.range.clone())?;
        }
        Commands::Scale(args) => {
            midiedit_cli::scale(args.file.clone(), args.scale, args.center, args.offset, args.range.clone())?;
        }
    }

    Ok(())
}
