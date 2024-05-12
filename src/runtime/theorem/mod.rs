use crate::audiotheorem::runtime::theorem::{Sequence, Tone};

#[derive(Clone, Debug)]
pub struct Theorem { // This is Simply an interface for controlling Subsequences
    size: u8, // This is the size of the sequence, for easy access
    // Todo: Add indicator for 'Tonic' and Intervals in relation to the tonic, and then create inversion module. this will be the cursor <also proprietary Nexus, Ancillary, 2024>
    pub voicings: Vec<Sequence>,
}

// Stores a Vector of Tones, and their associated Chords
impl Theorem {
    // We start and end with a blank slate
    pub fn new() -> Theorem { Theorem { size: 0, voicings: Vec::new() } }

    fn add(&mut self, index: u8, velocity: u8) {
        self.size += 1;

        // When we try to add a note here, it will succeed if it 'fits' in the current sequence
        if step_sequencer.last().play_note(index, velocity) { return; }

        // and if it fails we need to spawn a new sequence
        // This is when we go beyond bounds of +/- 7 per 12 == 13 == 14 // 2
        // and should it also be for min and max being less than a 24 ? or 
        // should it split when it exceeds a 7th or 14th interval?
        self.voicings.push(Sequence::new());
        self.voicings.last().play_note(index, velocity);
    }

    fn find_pitch_groups(&mut self) {

        
        let average_velocity = self.tones.iter().map(|t| t.velocity()).sum::<u8>() / self.tones.len() as u8;

        // PLAYED KEYS //
        /////////////////
        // We can take all the notes that we have played and iterate +/- 12 to populate our Sequence Data types
        for tone in self.tones.iter() {
            let index = tone.index();
            let velocity = tone.velocity(); // In all reality a tone could have a disposition as well.. and our matrix and tone can merge into a ToneMatrix
            
        
        }


    }
    pub fn tones(&self) -> Vec<Tone> { self.tones.clone() }
    pub fn get_tone(index: u8, velocity: u8) -> Option<Tone> { Some(Tone::from_iv(index, velocity)) }

    fn delete_tone(&mut self, index: u8) {
        if self.size == 0 { return; }

        for sub in self.sequences.iter() {
            if ele.remove(index) { self.size -= 1; return; }
        }

    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 
            { self.add_tone(index, velocity); } 
        else 
            { self.delete_tone(index); }
        
        // TODO: Add variable Debugger
        //self.construct_chords(); // This works, but takes up a lot of the output buffer
        self.find_pitch_groups();
    }

    pub fn print_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("=========================");
        println!("!!! Audio Theorem GUI !!!");
        println!("=========================\n");
        println!("{:#?}", *self);
    }
}