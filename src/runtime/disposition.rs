// Disposition is a Music Theory concept that is used to determine the color of the note (later might add a fractional degree or mixing of colors as a vector)
// Copyrigtht (c) 2024 by Richard I Christopher, Big Stick Studio - All Rights Reserved, Proprietary License - The NEXUS Project

#[derive(Debug)]
pub enum Disposition {
    Natural,            // Silent - Not being played
    Played,             // Will appear Blue
    Harmonious,         // Will appear Green if these notes are a uniform value amongst the top pitchgroups
    Mediant,            // Will appear Orange if these notes are a non-uniform value amongst the top pitchgroups
    Dissident,          // Will appear Red if these notes are a non-uniform value amongst the top pitchgroups
}

impl Disposition {
    pub fn from_u8(disposition: u8) -> Disposition {
        match disposition {
            1 => Disposition::Played,
            2 => Disposition::Harmonious,
            4 => Disposition::Mediant,
            8 => Disposition::Dissident,
            _ => Disposition::Natural
        }
    }

    pub fn as_u32(&self) -> u32 {
        match *self {
            Disposition::Natural => 0,
            Disposition::Played => 1,
            Disposition::Harmonious => 2,
            Disposition::Mediant => 4,
            Disposition::Dissident => 8

        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Disposition::Natural => 0,
            Disposition::Played => 1,
            Disposition::Harmonious => 2,
            Disposition::Mediant => 4,
            Disposition::Dissident => 8,
        }
    }
}

