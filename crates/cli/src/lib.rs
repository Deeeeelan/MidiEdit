use anyhow::{Context, Ok, Result};
use midiedit_core::RangeArgs;
use midiedit_edit_engine;
use midly::{MetaMessage, MidiMessage, Smf, TrackEventKind};

pub fn test() {
    println!("works yay")
}

fn read_midi_file(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let data =
        std::fs::read(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    Ok(data)
}

pub fn read_file(path: std::path::PathBuf) -> Result<()> {
    let data = read_midi_file(&path)?;
    println!("{:?} exists!", path);

    let smf = Smf::parse(&data).with_context(|| format!("could not parse file"))?;

    println!("{} tracks", smf.tracks.len());

    let mut tempo = 120;
    let mut found_tempo = false;

    for track in smf.tracks {
        for event in track {
            if !found_tempo && let TrackEventKind::Meta(msg) = event.kind {
                if let MetaMessage::Tempo(tempo_meta) = msg {
                    let ms_tempo = tempo_meta.as_int(); // Tempo is represented in microseconds/quarter note
                    let real_tempo = 60000000 / ms_tempo;
                    tempo = real_tempo;
                    found_tempo = true;
                    break;
                }
            } else if let TrackEventKind::Midi {
                channel: _,
                message: msg,
            } = event.kind
            {
                if let MidiMessage::NoteOn { key, vel: _ } = msg {
                    println!("Key {:?} played", key)
                }
            }
        }
    }

    println!("The starting tempo of the track is: {}", tempo);

    Ok(())
}

// TODO: Acutally make this the cli thing

pub fn transpose(path: std::path::PathBuf, distance: i8, range_args: RangeArgs) -> Result<()> {
    midiedit_edit_engine::transpose(path, distance, range_args)?;
    Ok(())
}

pub fn scale(
    path: std::path::PathBuf,
    scale: f64,
    center: i8,
    offset: i8,
    range_args: RangeArgs,
) -> Result<()> {
    midiedit_edit_engine::scale(path, scale, center, offset, range_args)?;
    Ok(())
}
