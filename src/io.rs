use std::{fs::File, io::Write};

use crate::instrument::Note;

pub trait Audio {
    fn save(filename: &str, notes: &mut Vec<Note>) -> std::io::Result<()>;
}

pub struct WAV {}

impl WAV {
    pub const SAMPLE_RATE: u32 = 44100;
    pub const BITS_PER_SAMPLE: u16 = 16;
    pub const NUM_OF_CHANNELS: u16 = 1;
}

impl Audio for WAV {
    fn save(filename: &str, notes: &mut Vec<Note>) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        // RIFF header
        file.write(b"RIFF")?;
        file.write(&36u32.to_le_bytes())?;
        file.write(b"WAVE")?;

        // "fmt " subchunk
        file.write(b"fmt ")?;
        file.write(&16u32.to_le_bytes())?;
        file.write(&1u16.to_le_bytes())?;
        file.write(&Self::NUM_OF_CHANNELS.to_le_bytes())?; // Moro ? Stereo ?
        file.write(&Self::SAMPLE_RATE.to_le_bytes())?;
        file.write(
            &(Self::SAMPLE_RATE * Self::NUM_OF_CHANNELS as u32 * Self::BITS_PER_SAMPLE as u32 / 8)
                .to_le_bytes(),
        )?;
        file.write(&(Self::NUM_OF_CHANNELS * Self::BITS_PER_SAMPLE / 8).to_le_bytes())?;
        file.write(&Self::BITS_PER_SAMPLE.to_le_bytes())?;

        // TODO: Write Data
        let mut i = 0.0;
        let mut buffer: Vec<f32> = vec![];
        let mut max_value = 0.0;

        loop {
            let t: f32 = i / Self::SAMPLE_RATE as f32;

            if notes.iter().all(|e| !e.is_active(t)) {
                break;
            }

            let v = notes.iter_mut().fold(0.0, |prev, n| prev + n.play(t));

            if v.abs() > max_value {
                max_value = v.abs();
            }

            buffer.push(v);

            i += 1.0;
        }

        // Normalize all volume
        buffer.iter_mut().for_each(|v| *v /= max_value);

        // "data" subchunk
        file.write(b"data")?;
        file.write(
            &(buffer.len() as u32 * Self::NUM_OF_CHANNELS as u32 * Self::BITS_PER_SAMPLE as u32
                / 8)
            .to_le_bytes(),
        )?;

        file.write(
            buffer
                .iter()
                .map(|v| v * 2f32.powf(Self::BITS_PER_SAMPLE as f32 - 1.0))
                .map(|v| (v as i16).to_le_bytes())
                .flatten()
                .collect::<Vec<_>>()
                .as_slice(),
        )?;

        Ok(())
    }
}
