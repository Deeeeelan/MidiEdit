use std::{ops::Mul, path};

use anyhow::{Context, Ok, Result};
use midiedit_core::RangeArgs;
use midly::{
    Header, MetaMessage, MidiMessage, PitchBend, Smf, TrackEventKind,
    num::{u7, u28},
};

fn read_file(path: &std::path::PathBuf) -> Result<Vec<u8>> {
    let data =
        std::fs::read(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    Ok(data)
}

fn parse_midi_file(data: &[u8]) -> Result<Smf<'static>> {
    let smf = Smf::parse(data)
        .with_context(|| "could not parse file".to_string())?
        .make_static();
    Ok(smf)
}

fn midi_to_smf(path: &std::path::PathBuf) -> Result<Smf<'static>> {
    let data = read_file(&path)?;
    return parse_midi_file(&data);
}

struct ProcessedSmf<'a> {
    header: Header,
    tracks: Vec<AbsTrack<'a>>,
}

struct AbsTrack<'a>(Vec<AbsEvent<'a>>);

struct AbsEvent<'a> {
    abs_time: u64,
    event: AbsEventKind<'a>,
}
enum AbsEventKind<'a> {
    Midi {
        channel: u8,
        message: CompactMidiMessage,
    },
    SysEx(&'a [u8]),
    Escape(&'a [u8]),
    Meta(MetaMessage<'a>),
}

enum CompactMidiMessage {
    Note(Note),
    Aftertouch { key: u7, vel: u7 },
    Controller { controller: u7, value: u7 },
    ProgramChange { program: u7 },
    ChannelAftertouch { vel: u7 },
    PitchBend { bend: PitchBend },
}
struct Note {
    key: u7,
    note_length: u64,
    start_vel: u7,
    end_vel: u7,
} // Note that the velocity for the NoteOff message is used in some cases

/// used for the pairing of NoteOn/NoteOff events
struct NoteStartFragment {
    key: u7,
    start_time: u64,
    start_vel: u7,
}

/// Processes the smf to a usable format
fn process_smf(smf: Smf) {
    // -> ProcessedSmf {
    let mut curr_time: u64 = 0;
    let mut new_smf: ProcessedSmf = ProcessedSmf {
        header: smf.header,
        tracks: Vec::new(),
    };
    let new_tracks = &new_smf.tracks;
    for track in smf.tracks.iter() {
        if !track.is_empty() {
            let mut notes: Vec<Note> = vec![]; // Need to move these inside of CompactMidiMessage
            let mut active: [Vec<NoteStartFragment>; 128] = [const { Vec::new() }; 128];
            for event in track {
                curr_time += event.delta.as_int() as u64;
                if let TrackEventKind::Midi { channel, message } = event.kind {
                    if let MidiMessage::NoteOn { key, vel } = message {
                        if vel > 0 {
                            active[key.as_int() as usize].push(NoteStartFragment {
                                key,
                                start_time: curr_time,
                                start_vel: vel,
                            })
                        } else if active[key.as_int() as usize].last().is_some() {
                            let start_data = active[key.as_int() as usize].pop().unwrap();
                            let note = Note {
                                key,
                                note_length: curr_time - start_data.start_time,
                                start_vel: start_data.start_vel,
                                end_vel: vel,
                            };
                            notes.push(note);
                        }
                    } else if let MidiMessage::NoteOff { key, vel } = message
                        && active[key.as_int() as usize].last().is_some()
                    {
                        // If there is no pairing start event, skip it
                        let start_data = active[key.as_int() as usize].pop().unwrap();
                        let note = Note {
                            key,
                            note_length: curr_time - start_data.start_time,
                            start_vel: start_data.start_vel, // TODO: Move Notes into CompactMidiMessage and AbsEventKind, pass down misc msgs
                            end_vel: vel,
                        };
                        notes.push(note);
                    }
                }
            }
        }
    }
}

/// Transporm a specified region
fn transform_smf_region(
    // TODO: turn this into a parsing function to process this into a representative state so I don't have to deal with deltatime
    func: impl for<'a> Fn(&mut u7, &mut u7),
    smf: &mut Smf,
    range_args: RangeArgs,
) {
    let mut curr_time: u64 = 0;
    for (i, track) in smf.tracks.iter_mut().enumerate() {
        if range_args.track.contains(&i) || range_args.track.is_empty() {
            let mut notes: Vec<Note> = vec![];
            let mut active: [Vec<(u64, &mut u7, &mut u7)>; 128] = [const { Vec::new() }; 128];
            for event in track {
                curr_time += event.delta.as_int() as u64;
                if range_args.start.map_or(true, |start| curr_time >= start) {
                    if let TrackEventKind::Midi {
                        channel: _,
                        message: msg,
                    } = &mut event.kind
                    {
                        if let CompactMidiMessage::NoteOn { key, vel } = msg {
                            if *vel > 0 {
                                active[key.as_int() as usize].push((curr_time, key, vel))
                            } else if active[key.as_int() as usize].last().is_some() {
                                let start_data = active[key.as_int() as usize].pop().unwrap();
                                let note = Note {
                                    start: start_data,
                                    end: (curr_time, key, vel),
                                };
                                notes.push(note);
                            }
                        } else if let CompactMidiMessage::NoteOff { key, vel } = msg
                            && active[key.as_int() as usize].last().is_some()
                        {
                            // If there is no pairing start event, skip it
                            let start_data = active[key.as_int() as usize].pop().unwrap();
                            let note = Note {
                                start: start_data,
                                end: (curr_time, key, vel),
                            };
                            notes.push(note);
                        }
                    }
                    if let Some(end) = range_args.end
                        && curr_time > end
                    {
                        break;
                    }
                }
            }
            for n in notes.iter_mut() {
                if range_args.start.map_or(true, |s| n.end.0 > s)
                    && range_args.end.map_or(true, |e| n.start.0 < e)
                {
                    func(n.start.1, n.start.2);
                    func(n.end.1, n.end.2);
                }
            }
        }
    }
}

/// applies a given function to all the notes within range of the smf
fn transpose_smf_region(smf: &mut Smf, distance: i8, range_args: RangeArgs) {
    let transpose_note = |key: &mut u7, _vel: &mut u7| {
        *key = u7::new(((key.as_int() as i8).saturating_add(distance)) as u8);
    };

    transform_smf_region(transpose_note, smf, range_args);
}

/// applies velocity scaling
fn scale_smf_region(smf: &mut Smf, scale: f64, center: i8, offset: i8, range_args: RangeArgs) {
    let scale_note = |_key: &mut u7, vel: &mut u7| {
        *vel = u7::new(
            (((vel.as_int() as i8).saturating_sub(center) as f64)
                .mul(scale)
                .round() as i8)
                .saturating_add(offset) as u8,
        );
    };

    transform_smf_region(scale_note, smf, range_args);
}

pub fn transpose(path: std::path::PathBuf, distance: i8, range_args: RangeArgs) -> Result<()> {
    let data = read_file(&path)?;
    let mut smf = parse_midi_file(&data)?;

    println!("Transposing by {:?}", distance);
    transpose_smf_region(&mut smf, distance, range_args);

    smf.save(&path)?;

    Ok(())
}

pub fn scale(
    path: std::path::PathBuf,
    scale: f64,
    center: i8,
    offset: i8,
    range_args: RangeArgs,
) -> Result<()> {
    let data = read_file(&path)?;
    let mut smf = parse_midi_file(&data)?;

    println!("Scaling by {:?}", scale);
    scale_smf_region(&mut smf, scale, center, offset, range_args);

    smf.save(&path)?;

    Ok(())
}
