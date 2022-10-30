// Piano Rolls (Inspired from DAWs like FL Studio and Ableton)
#[derive(Debug, Clone, Copy)]
pub struct Roll {
    pub v: f32,
}

// impl Deref for Roll {
//     type Target = f32;
//     fn deref(&self) -> &Self::Target {
//         &self.v
//     }
// }

impl Roll {
    const TEMPO: f32 = 84.0;
    const TIME_SIGNATURE: f32 = 1.0 / 4.0;

    pub fn new(v: f32) -> Self {
        Self { v }
    }

    pub fn seconds(&self) -> f32 {
        self.v * Self::TIME_SIGNATURE * 60.0 / Self::TEMPO
    }
}

impl std::ops::Add<Roll> for Roll {
    type Output = Self;

    fn add(self, rhs: Roll) -> Self::Output {
        Roll::new(self.v + rhs.v)
    }
}
