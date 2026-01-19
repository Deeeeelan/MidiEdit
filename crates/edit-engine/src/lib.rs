use midly::{MidiMessage, Smf, TrackEventKind, num::u7};
use anyhow::{Context, Ok, Result};

fn read_midi_file(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let data = std::fs::read(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    Ok(data)
}

pub fn transpose(path: std::path::PathBuf, distance: i8, start: i32, end: i32) -> Result<()>{
    let data = read_midi_file(&path)?;

    let mut smf = Smf::parse(&data)
        .with_context(|| format!("could not parse file"))?;

    println!("Transposing with distance {:?}", distance);

    for track in &mut smf.tracks {
        for event in track {
            if let TrackEventKind::Midi { channel: _, message: msg } = &mut event.kind {
                if let MidiMessage::NoteOn { key, vel: _ } = msg {
                    *key = u7::new(((key.as_int() as i8) + distance) as u8); // TODO: Add behavior when this goes out of range (0-127)
                }else if let MidiMessage::NoteOff { key, vel: _ } = msg {
                    *key = u7::new(((key.as_int() as i8) + distance) as u8);
                }
            }
        }
    }

    smf.save(&path)?;

    
    Ok(())
}