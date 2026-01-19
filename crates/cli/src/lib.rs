use midly::{Smf, TrackEventKind, MetaMessage};
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

    let smf = midly::Smf::parse(&data)
        .with_context(|| format!("could not parse file"))?;

    println!("{} tracks", smf.tracks.len());

    let mut tempo = 120;

    for track in smf.tracks {
        for event in track {
            if let TrackEventKind::Meta(msg) = event.kind {
                if let MetaMessage::Tempo(tempo_meta) = msg {
                    let ms_tempo = tempo_meta.as_int(); // Tempo is represented in microseconds/quarter note
                    let real_tempo = 60000000 / ms_tempo;
                    tempo = real_tempo;
                    break;
                }
            }
        }
    }

    println!("The starting tempo of the track is: {}", tempo);
    
    Ok(())
}