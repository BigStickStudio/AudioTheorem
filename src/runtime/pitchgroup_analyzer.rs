
use crate::types::{Note, PitchGroup};

struct PitchclassAnalyzer {
    pitchgroup: PitchGroup,
    probability: f64,
    members: Vec<Note>,
    offnotes: Vec<Note>
}