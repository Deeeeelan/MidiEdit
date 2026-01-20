use std::iter::repeat_with;

use midly::{MidiMessage, Smf, TrackEventKind, num::{u4, u7}};
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

struct Note<'a> { // currently in time, pitch, velocity pairs for now, probably not efficent but end velocity actually mattters...
    start: (u64, &'a mut u7, &'a mut u7),
    end: (u64, &'a mut u7, &'a mut u7),
}

/// Transporm a specified region
fn transform_smf_region(func: impl for <'a> Fn(&mut u7), smf: &mut Smf, range_args: RangeArgs) {
    let mut curr_time: u64 = 0;
    for track in &mut smf.tracks {
        let mut notes: Vec<Note> = vec![];
        let mut active: [Vec<(u64, &mut u7, &mut u7)>; 128] = [const {Vec::new()}; 128];
        for event in track {
            curr_time += event.delta.as_int() as u64;
            if range_args.start.map_or(true, |start| curr_time >= start) { 
                //TODO: implement tracks
                //TODO: Function for restreching notes (gl)
                if let TrackEventKind::Midi { channel: _, message: msg } = &mut event.kind {
                    if let MidiMessage::NoteOn { key, vel } = msg {
                        if *vel > 0 {
                            active[key.as_int() as usize].push((curr_time, key, vel))
                        } else {
                            if active[key.as_int() as usize].last() != None {
                                let start_data = active[key.as_int() as usize].pop().unwrap(); 
                                let note = Note {
                                    start: start_data,
                                    end: (curr_time, key, vel)
                                };
                                notes.push(note);
                            }
                        }
                   }else if let MidiMessage::NoteOff { key, vel } = msg {
                        if active[key.as_int() as usize].last() != None { // If there is no pairing start event, skip it
                            let start_data = active[key.as_int() as usize].pop().unwrap(); 
                            let note = Note {
                                start: start_data,
                                end: (curr_time, key, vel)
                            };
                            notes.push(note);
                        }
                        
                    }
                }
                if let Some(end) = range_args.end && curr_time > end { break; }
            }
        }
            for n in notes.iter_mut() { // this syntax is lowkenuinley crazy compared to python :sob:
                if let (Some(end), Some(st)) = (range_args.end, range_args.start) && (n.start.0 < end && n.end.0 > st) {
                    func(n.start.1);
                    func(n.end.1);
                }
            }
    }
}

/// applies a give functions to all the notes within range of the smf
fn transpose_smf_region(smf: &mut Smf, distance: i8, range_args: RangeArgs) {
    let transpose_note = |key: &mut u7| {
        *key = u7::new(((key.as_int() as i8).saturating_add(distance)) as u8);
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