use midly::{MidiMessage, Smf, TrackEventKind, num::u7};
use anyhow::{Context, Ok, Result};
use midiedit_core::{RangeArgs};

fn read_file(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let data = std::fs::read(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    Ok(data)
}

fn parse_midi_file(data: &[u8]) -> Result<Smf<'static>> {
    let smf = Smf::parse(&data)
        .with_context(|| format!("could not parse file"))?.make_static();
    Ok(smf)
}

// 
fn transform_smf_region(func: impl for <'a> Fn(&'a mut u7), smf: &mut Smf, range_args: RangeArgs) {
    let mut curr_time: u64 = 0;
    
    for track in &mut smf.tracks {
        for event in track {
            curr_time += event.delta.as_int() as u64;
            if (range_args.start != None && curr_time >= range_args.start.unwrap()) || range_args.start == None {
                println!("In start range: {:?}", curr_time)
            }
            if let TrackEventKind::Midi { channel: _, message: msg } = &mut event.kind {
                if let MidiMessage::NoteOn { key, vel: _ } = msg {
                    func(key)
                }else if let MidiMessage::NoteOff { key, vel: _ } = msg {
                    func(key)
                }
            }
        }
    }
}

/// applies a give functions to all the notes within range of the smf
fn transpose_smf_region(smf: &mut Smf, distance: i8, range_args: RangeArgs) {
    let transpose_note = |key: &mut u7| {
        *key = u7::new(((key.as_int() as i8) + distance) as u8);
    };

    transform_smf_region(transpose_note , smf, range_args);
    
}

pub fn transpose(path: std::path::PathBuf, distance: i8, range_args: RangeArgs) -> Result<()>{
    let data = read_file(&path)?;
    let mut smf = parse_midi_file(&data)?;

    println!("Transposing by {:?}", distance);
    transpose_smf_region(&mut smf, distance, range_args);

    smf.save(&path)?;
    
    Ok(())
}