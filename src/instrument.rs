use biquad::{Biquad, DirectForm1};

use crate::{pitch::Pitch, roll::Roll};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Generator {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    DC,
}

/*********************/
#[derive(Debug, Clone, Copy)]
pub struct Oscillator {
    pub generator: Generator,
    pub velocity: f32,
}

impl Oscillator {
    /*
     * Desmos: https://www.desmos.com/calculator/2xswrci3s0
     */
    pub fn play(&self, t: f32, f: f32) -> f32 {
        #[allow(non_snake_case)]
        let T = 1. / f;
        let m = t % T; // MOD

        // Generator
        let v: f32 = match self.generator {
            Generator::Sine => (2.0 * PI * f * t).sin(),
            Generator::Square => {
                if t % T < T / 2.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Generator::Triangle => {
                if m < T / 4.0 {
                    4.0 * f * (t % (T / 2.0))
                } else if m < 3. * T / 4.0 {
                    -4.0 * f * m + 2.
                } else {
                    4.0 * f * (t % (T / 2.0)) - 2.0
                }
            }
            Generator::Sawtooth => 2.0 * ((t - (T / 2.0)) % T) - 1.0,
            Generator::DC => 1.0,
        };

        v * self.velocity
    }
}

/*********************/

#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    pub attack_duration: f32,
    pub decay_duration: f32,
    pub decay_ratio: f32,
    pub release_duration: f32,
}

impl Envelope {
    fn play(&self, t: f32, note: &Note) -> f32 {
        let rt = t - note.start.seconds();
        if !self.is_active(t, note) {
            return 0.0;
        }

        if rt < self.attack_duration {
            // Attack
            return rt % self.attack_duration * self.decay_ratio / self.attack_duration;
        }

        if rt < self.attack_duration + self.decay_duration {
            // Decay
            return self.decay_ratio
                - (rt - self.attack_duration) % self.decay_duration / (2.0 * self.decay_duration);
        }

        if rt <= self.attack_duration + self.decay_duration + note.duration.seconds() {
            // Sustain
            return 1.0;
        }

        // Release
        -(rt - self.attack_duration
            - self.decay_duration
            - self.release_duration
            - note.duration.seconds())
            % self.release_duration
            / self.release_duration
    }

    fn is_active(&self, t: f32, note: &Note) -> bool {
        t < note.start.seconds()
            + note.duration.seconds()
            + self.attack_duration
            + self.decay_duration
            + self.release_duration
    }
}

/*********************/
#[derive(Debug, Clone)]
pub struct Instrument<'a> {
    pub oscillators: Vec<&'a Oscillator>,
    pub envelope: Option<&'a Envelope>,
    pub velocity: f32,
    pub filter: Option<DirectForm1<f32>>,
}

impl<'a> Instrument<'a> {
    pub fn play(&mut self, t: f32, f: f32, note: &Note) -> f32 {
        let mut v = self
            .oscillators
            .iter()
            .fold(0.0, |prev, o| prev + o.play(t, f));

        v *= match self.envelope {
            Some(e) => e.play(t, note),
            None => 1.0,
        };

        if let Some(filter) = &mut self.filter {
            v = filter.run(v);
        }

        v * self.velocity
    }

    pub fn is_active(&self, t: f32, note: &Note) -> bool {
        if let Some(e) = self.envelope {
            e.is_active(t, note)
        } else {
            false
        }
    }
}

/*********************/
#[derive(Debug, Clone)]
pub struct Note<'a> {
    pub pitch: Pitch,
    pub velocity: f32,
    pub start: Roll,
    pub duration: Roll,
    pub instrument: Instrument<'a>,
}

impl<'a> Note<'a> {
    pub fn new(
        pitch: Pitch,
        duration: f32,
        start: f32,
        instrument: Instrument<'a>,
        velocity: f32,
    ) -> Self {
        Self {
            pitch,
            velocity,
            start: Roll::new(start),
            duration: Roll::new(duration),
            instrument,
        }
    }

    pub fn play(&mut self, t: f32) -> f32 {
        if !self.is_active(t) {
            return 0.0;
        }

        // Note still havent started
        if t < self.start.seconds() {
            return 0.0;
        }

        self.instrument
            .play(t, self.pitch as u32 as f32, &self.clone())
            * self.velocity
    }

    pub fn is_active(&self, t: f32) -> bool {
        t < (self.start + self.duration).seconds() || self.instrument.is_active(t, self)
    }
}
