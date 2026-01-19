use midly;
use anyhow::{Context, Ok, Result};

pub fn test() {
    println!("works yay")
}

fn read_midi_file(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let data = std::fs::read(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    Ok(data)
}

pub fn read_file(path: std::path::PathBuf) -> Result<()>{

    
    let data = read_midi_file(&path)?;
    println!("{:?} exists!", path);

    let smf = midly::Smf::parse(&data).unwrap();

    println!("{} tracks", smf.tracks.len());
    
    Ok(())
}