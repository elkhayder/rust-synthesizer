/*
* Port from Cpp : https://github.com/OneLoneCoder/Javidx9/blob/master/PixelGameEngine/SmallerProjects/OneLoneCoder_PGE_MIDI.cpp
*/

use std::{
    fs::File,
    io::{Read, Seek},
};

// Regular Events type
#[derive(Debug, Clone)]
enum MIDIEventType {
    NoteOff,
    NoteOn,
    Other,
}

// Regular Events name
#[derive(Debug, Clone)]
enum MIDIEventName {
    VoiceNoteOff = 0x80,
    VoiceNoteOn = 0x90,
    VoiceAftertouch = 0xA0,
    VoiceControlChange = 0xB0,
    VoiceProgramChange = 0xC0,
    VoiceChannelPressure = 0xD0,
    VoicePitchBend = 0xE0,
    SystemExclusive = 0xF0,
}

impl PartialEq<u8> for MIDIEventName {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == (*other & 0xF0)
    }

    fn ne(&self, other: &u8) -> bool {
        !self.eq(other)
    }
}

// Meta Events Name
#[derive(Debug, Clone)]
enum MIDIMetaEventName {
    MetaSequence = 0x00,
    MetaText = 0x01,
    MetaCopyright = 0x02,
    MetaTrackName = 0x03,
    MetaInstrumentName = 0x04,
    MetaLyrics = 0x05,
    MetaMarker = 0x06,
    MetaCuePoint = 0x07,
    MetaChannelPrefix = 0x20,
    MetaEndOfTrack = 0x2F,
    MetaSetTempo = 0x51,
    MetaSMPTEOffset = 0x54,
    MetaTimeSignature = 0x58,
    MetaKeySignature = 0x59,
    MetaSequencerSpecific = 0x7F,
    Other,
}

impl PartialEq<u8> for MIDIMetaEventName {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }

    fn ne(&self, other: &u8) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Clone)]
struct MIDIEvent {
    event: MIDIEventType,
    key: u8,
    velocity: u8,
    delta_tick: u32,
}

#[derive(Debug, Clone)]
struct MIDINote {
    key: u8,
    velocity: u8,
    start_time: i32,
    duration: i32,
}

#[derive(Debug, Clone)]
struct MIDITrack {
    name: Option<&'static str>,
    instrument: Option<&'static str>,
    events: Vec<MIDIEvent>,
    notes: Vec<MIDINote>,
}

impl MIDITrack {
    pub fn new() -> Self {
        Self {
            name: None,
            instrument: None,
            events: vec![],
            notes: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct MIDIFile {
    tracks: Vec<MIDITrack>,
}

impl MIDIFile {
    pub fn parse(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;

        let mut instance = Self { tracks: vec![] };

        let file_id = Self::read_u32(&mut file);
        let header_length = Self::read_u32(&mut file);
        let format = Self::read_u16(&mut file);
        let track_chunks = Self::read_u16(&mut file);
        let division = Self::read_u16(&mut file);

        println!(
            " File id = {}\n Header length = {} \n Format = {} \n Track chunks = {}\n Division = {}",
            file_id, header_length, format, track_chunks, division
        );

        for chunk in 0..track_chunks as usize {
            println!("--- New Chunk ---");

            let track_id = Self::read_u32(&mut file);
            let track_length = Self::read_u32(&mut file);

            println!("ID = {}, Length = {}", track_id, track_length);

            let mut is_end_of_track = false;
            let mut wall_time = 0;
            let mut prev_status: u8 = 0;

            instance.tracks.push(MIDITrack::new());

            while !is_end_of_track && {
                let rb = file.read(&mut [0u8; 1]);
                let r = match rb {
                    Ok(b) => b != 0,
                    Err(_) => false,
                };
                let _ = file.seek(std::io::SeekFrom::Current(-1));
                r
            } {
                let mut status = Self::read_u8(&mut file);

                let status_delta_time = Self::read_value(&mut file);

                println!("Status = {:#X}, Δt = {}", status, status_delta_time);

                if status < 0x80 {
                    status = prev_status;

                    let _ = file.seek(std::io::SeekFrom::Current(-1));
                }

                if MIDIEventName::VoiceNoteOff == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let note = Self::read_u8(&mut file);
                    let velocity = Self::read_u8(&mut file);

                    instance.tracks[chunk].events.push(MIDIEvent {
                        event: MIDIEventType::NoteOff,
                        key: note,
                        velocity,
                        delta_tick: status_delta_time,
                    });
                } else if MIDIEventName::VoiceNoteOn == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let note = Self::read_u8(&mut file);
                    let velocity = Self::read_u8(&mut file);

                    instance.tracks[chunk].events.push(MIDIEvent {
                        event: if velocity == 0 {
                            MIDIEventType::NoteOff
                        } else {
                            MIDIEventType::NoteOn
                        },
                        key: note,
                        velocity,
                        delta_tick: status_delta_time,
                    });
                } else if MIDIEventName::VoiceAftertouch == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let note = Self::read_u8(&mut file);
                    let velocity = Self::read_u8(&mut file);
                } else if MIDIEventName::VoiceControlChange == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let control_id = Self::read_u8(&mut file);
                    let control_value = Self::read_u8(&mut file);
                } else if MIDIEventName::VoiceProgramChange == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let prog_id = Self::read_u8(&mut file);
                } else if MIDIEventName::VoiceChannelPressure == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let pressure = Self::read_u8(&mut file);
                } else if MIDIEventName::VoicePitchBend == status {
                    prev_status = status;
                    let channel = Self::read_u8(&mut file);
                    let nLS7B = Self::read_u8(&mut file);
                    let nMS7B = Self::read_u8(&mut file);
                } else if MIDIEventName::SystemExclusive == status {
                    prev_status = 0;

                    if status == 0xFF {
                        let x_type = Self::read_u8(&mut file);
                        let length = Self::read_value(&mut file);

                        if MIDIMetaEventName::MetaSequence == x_type {
                            println!(
                                "Sequence Number: {} {}",
                                Self::read_u8(&mut file),
                                Self::read_u8(&mut file)
                            );
                        } else if MIDIMetaEventName::MetaText == x_type {
                            println!("Text: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaCopyright == x_type {
                            println!("Copyright: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaTrackName == x_type {
                            instance.tracks[chunk].name =
                                Some(Self::read_string(&mut file, length));
                            println!(
                                "Track Name: {}",
                                instance.tracks[chunk].name.unwrap_or("Unknown")
                            );
                        } else if MIDIMetaEventName::MetaInstrumentName == x_type {
                            instance.tracks[chunk].instrument =
                                Some(Self::read_string(&mut file, length));
                            println!(
                                "Instrument Name: {}",
                                instance.tracks[chunk].instrument.unwrap_or("Unknown")
                            );
                        } else if MIDIMetaEventName::MetaLyrics == x_type {
                            println!("Lyrics: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaMarker == x_type {
                            println!("Marker: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaCuePoint == x_type {
                            println!("Cue: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaChannelPrefix == x_type {
                            println!("Prefix: {}", Self::read_string(&mut file, length));
                        } else if MIDIMetaEventName::MetaEndOfTrack == x_type {
                            is_end_of_track = true;
                        }
                    }
                } else {
                    println!("Unrecognised Status Byte: {:#X}", status);
                }
            }
        }

        // file.stre

        Ok(instance)
    }

    fn read_u8(file: &mut File) -> u8 {
        let mut n8 = [0u8; 1];
        file.read_exact(&mut n8);
        n8[0]
    }

    fn read_u16(file: &mut File) -> u16 {
        let mut n16 = [0u8; 2];
        file.read_exact(&mut n16);
        u16::from_be_bytes(n16)
    }

    fn read_u32(file: &mut File) -> u32 {
        let mut n32 = [0u8; 4];
        file.read_exact(&mut n32);
        u32::from_be_bytes(n32)
    }

    fn read_value(file: &mut File) -> u32 {
        let mut value: u32 = 0;
        let mut buf: u8 = 0;

        loop {
            buf = Self::read_u8(file);

            value = ((value & 0x7F) << 7) | (buf & 0x7F) as u32;

            if buf & 0x80 != 0 {
                break;
            }
        }

        value
    }

    fn read_string(file: &mut File, length: u32) -> &'static str {
        let mut buf: Vec<u8> = vec![];

        for _ in 0..length as usize {
            buf.push(Self::read_u8(file));
        }

        Box::leak(
            String::from_utf8_lossy(buf.as_slice())
                .to_string()
                .into_boxed_str(),
        )
    }
}
