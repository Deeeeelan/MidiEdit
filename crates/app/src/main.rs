use clap::{Args, Parser, Subcommand};
use anyhow::{Context, Result};
use midiedit_cli;

/// A Simple Midi Editor
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// reads and prints the file
    Read(ReadFile),

    /// test if the file is a midi file and print some debug data
    Test(TestFile),

    /// tramspose all the tracks of a midi file
    Transpose(TransposeFile),
}

#[derive(Args)]
struct RangeArgs {
    /// Which track to apply the transformation
    #[arg(short, long)]
    track: u8,

    /// where to start the transformation
    #[arg(short, long)]
    start: u32,

    /// where to end the transformation
    #[arg(short, long)]
    end: u32,
}

/// reads and prints the file
#[derive(Args)]
struct TestFile {
    /// path of file to check
    file: std::path::PathBuf
}

/// test if the file is a midi file and print some debug data
#[derive(Args)]
struct ReadFile {
    /// path of file to read
    file: std::path::PathBuf,
}

/// tramspose all the tracks of a midi file
#[derive(Args)]
struct TransposeFile {
    /// path of file to transpose
    file: std::path::PathBuf,

    /// number of semitones to transpose by
    amt: i8,

    #[command(flatten)]
    range: RangeArgs,
}

/// scale the velocity levels of all of the tracks in a file
#[derive(Args)]
struct ScaleLevels {
    /// path of file to scale
    file: std::path::PathBuf,
    
    #[command(flatten)]
    range: RangeArgs,
}

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
            midiedit_cli::transpose(args.file.clone(),args.amt,0,0)?;
        }
    }

    Ok(())
}
