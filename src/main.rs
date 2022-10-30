use biquad::{Coefficients, DirectForm1, ToHertz, Q_BUTTERWORTH_F32};
use instrument::{Envelope, Generator, Instrument, Note, Oscillator};
use midi::MIDIFile;
use pitch::Pitch;

use crate::io::{Audio, WAV};

mod instrument;
mod io;
mod midi;
mod pitch;
mod roll;

fn main() {
    println!("{:?}", MIDIFile::parse("untitled.mid").unwrap());

    let ins = Instrument {
        oscillators: vec![
            &Oscillator {
                generator: Generator::Sine,
                velocity: 0.5,
            },
            &Oscillator {
                generator: Generator::Square,
                velocity: 0.1,
            },
            &Oscillator {
                generator: Generator::Triangle,
                velocity: 0.25,
            },
            &Oscillator {
                generator: Generator::Sawtooth,
                velocity: 0.15,
            },
        ],
        envelope: Some(&Envelope {
            attack_duration: 0.02,
            decay_duration: 0.05,
            decay_ratio: 1.5,
            release_duration: 0.05,
        }),
        velocity: 0.7,
        filter: None,
    };

    let chords_instrument = &Instrument {
        oscillators: vec![
            &Oscillator {
                generator: Generator::Sine,
                velocity: 0.5,
            },
            &Oscillator {
                generator: Generator::Square,
                velocity: 0.3,
            },
            &Oscillator {
                generator: Generator::Triangle,
                velocity: 0.25,
            },
            &Oscillator {
                generator: Generator::Sawtooth,
                velocity: 0.13,
            },
        ],
        envelope: Some(&Envelope {
            attack_duration: 0.06,
            decay_duration: 0.1,
            decay_ratio: 1.5,
            release_duration: 0.3,
        }),
        velocity: 0.8,
        filter: Some(DirectForm1::<f32>::new(
            Coefficients::<f32>::from_params(
                biquad::Type::LowPass,
                (WAV::SAMPLE_RATE as f32).hz(),
                0.3.khz(),
                Q_BUTTERWORTH_F32,
            )
            .unwrap(),
        )),
    };

    let mut notes: Vec<Note> = vec![];

    notes.append(&mut vec![
        Note::new(Pitch::C2, 8.0 * 3.5, 0.0, chords_instrument.clone(), 1.0),
        Note::new(Pitch::C1, 8.0 * 3.5, 0.0, chords_instrument.clone(), 1.0),
        /**************/
        Note::new(Pitch::E2, 8.0 * 3.5, 32.0, chords_instrument.clone(), 1.0),
        Note::new(Pitch::E1, 8.0 * 3.5, 32.0, chords_instrument.clone(), 1.0),
        /**************/
        Note::new(Pitch::C2, 8.0 * 3.0, 64.0, chords_instrument.clone(), 1.0),
        Note::new(Pitch::C1, 8.0 * 3.0, 64.0, chords_instrument.clone(), 1.0),
        Note::new(Pitch::D2, 1.0, 92.0, chords_instrument.clone(), 1.4),
        Note::new(Pitch::C1, 1.0, 92.0, chords_instrument.clone(), 1.4),
        /**************/
        Note::new(Pitch::E2, 8.0 * 3.5, 96.0, chords_instrument.clone(), 1.0),
        Note::new(Pitch::E1, 8.0 * 3.5, 96.0, chords_instrument.clone(), 1.0),
    ]);

    for n in 0..16 {
        for (i, p) in [
            Pitch::C4,
            Pitch::E4,
            Pitch::G4,
            Pitch::B4,
            Pitch::C5,
            Pitch::B4,
            Pitch::G4,
            Pitch::E4,
        ]
        .iter()
        .enumerate()
        {
            notes.push(Note::new(
                *p,
                1.0,
                i as f32 * 1.0 + n as f32 * 8.0,
                ins.clone(),
                0.7,
            ));
        }
    }

    let _ = WAV::save("output.wav", &mut notes);
}
