use anyhow::{Context, Result};
use clap::Parser;
use midiedit_cli;
use midiedit_core::{Cli, CliCommands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        CliCommands::Read(read_file) => {
            let content = std::fs::read_to_string(&read_file.file)
                .with_context(|| format!("could not read file `{}`", read_file.file.display()))?;

            println!("IT WORKS! path: {:?}", read_file.file);

            for line in content.lines() {
                println!("{}", line);
            }
        }
        CliCommands::Test(file_path) => {
            midiedit_cli::read_file(file_path.file.clone())?;
        }
        CliCommands::Play(file_path) => {
            // midiedit_playback::play_file(file_path.file.clone())?;
        }
        CliCommands::Transpose(args) => {
            midiedit_cli::transpose(args.file.clone(), args.amt, args.range.clone())?;
        }
        CliCommands::Scale(args) => {
            midiedit_cli::scale(
                args.file.clone(),
                args.scale,
                args.center,
                args.offset,
                args.range.clone(),
            )?;
        }
    }

    Ok(())
}
