use clap::{Args, Parser, Subcommand};

/// A Simple Midi Editor
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// reads and prints the file
    Read(ReadFile),

    /// test if the file is a midi file and print some debug data
    Test(TestFile),

    /// tramspose part of a midi file
    Transpose(TransposeFile),

    /// scale the velocity of the midi file
    Scale(ScaleLevels),
}

#[derive(Clone)]
#[derive(Args)]
pub struct RangeArgs {
    /// Which track to apply the transformation
    #[arg(short, long, value_delimiter = ' ', value_delimiter = '-')]
    pub track: Vec<isize>,

    /// where to start the transformation
    #[arg(short, long)]
    pub start: Option<u64>,

    /// where to end the transformation
    #[arg(short, long)]
    pub end: Option<u64>,
}

/// reads and prints the file
#[derive(Args)]
pub struct TestFile {
    /// path of file to check
    pub file: std::path::PathBuf
}

/// test if the file is a midi file and print some debug data
#[derive(Args)]
pub struct ReadFile {
    /// path of file to read
    pub file: std::path::PathBuf,
}

/// tramspose all the tracks of a midi file
#[derive(Args)]
pub struct TransposeFile {
    /// path of file to transpose
    pub file: std::path::PathBuf,

    /// number of semitones to transpose by
    pub amt: i8,

    #[command(flatten)]
    pub range: RangeArgs,
}

/// scale the velocity levels of all of the tracks in a file
#[derive(Args)]
pub struct ScaleLevels {
    /// path of file to scale
    pub file: std::path::PathBuf,

    /// Decimal
    pub scale: f64, // TODO: add default values
    
    pub center: i8,

    pub offset: i8,
    
    #[command(flatten)]
    pub range: RangeArgs,
}