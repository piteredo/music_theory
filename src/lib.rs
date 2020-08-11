const OCTAVE_SCALE_LEN: u32 = 7;
const OCTAVE_CHROMATIC_LEN: u32 = 12;

pub enum Step {
    C, D, E, F, G, A, B,
}
impl Step {
    fn as_u32(&self) -> u32 {
        match self {
            Step::C => 0,
            Step::D => 1,
            Step::E => 2,
            Step::F => 3,
            Step::G => 4,
            Step::A => 5,
            Step::B => 6,
        }
    }
}

pub enum Alter {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}
impl Alter {
    fn as_i32(&self) -> i32 {
        match self {
            Alter::DoubleFlat => -2,
            Alter::Flat => -1,
            Alter::Natural => 0,
            Alter::Sharp => 1,
            Alter::DoubleSharp => 2,
        }
    }
}

pub struct Octave(u32);
impl Octave {
    pub fn to_scale_steps(&self) -> u32 {
        self.0 * OCTAVE_SCALE_LEN
    }
}

pub enum KeyQuality {
    Major, Minor,
}

pub struct Pitch {
    pub step: Step,
    pub alter: Alter,
}

pub struct Note {
    pub pitch: Pitch,
    pub octave: Octave,
}
impl Note {
    pub fn scale_interval(&self, other: Note) -> Interval {
        let prefix =
        let sufix = self.scale_interval_sufix(other);
    }

    fn scale_interval_sufix(&self, other: Note) -> u32 {
        let self_step = self.abs_scale_step();
        let other_step = other.abs_scale_step();
        (self_step as i32 - other_step as i32).abs() as u32
    }

    fn abs_scale_step(&self) -> u32 {
        self.pitch.step.as_u32() + self.octave.to_scale_steps()
    }
}

pub struct Key {
    pub tonic_pitch: Pitch,
    pub key_quality: KeyQuality,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_note() {
        let pitch = Pitch {
            step: Step::C,
            alter: Alter::Natural,
        };
        let _note = Note {
            pitch,
            octave: Octave(4),
        };
    }

    #[test]
    fn init_key() {
        let pitch = Pitch {
            step: Step::C,
            alter: Alter::Natural,
        };
        let _key = Key {
            tonic_pitch: pitch,
            key_quality: KeyQuality::Major,
        };
    }

    #[test]
    fn scale_interval() {
        let note1 = Note {
            pitch: Pitch{ step: Step::G, alter: Alter::Natural },
            octave: Octave(4),
        };
        let note2 = Note {
            pitch: Pitch{ step: Step::D, alter: Alter::Sharp },
            octave: Octave(5),
        };
        note1.scale_interval(note2);
    }
}
