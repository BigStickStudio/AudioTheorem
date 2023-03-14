#[derive(Copy, Clone, Debug)]
pub struct MidiNotes {
    index: [u8; 0],
    velocity: [u8; 0]
}

impl MidiNotes{
    pub fn push(&mut self, index: u8, velocity: u8) {
        self.index.push(index);
        self.velocity.push(velocity);
    }

    pub fn pop(&mut self) -> Option<(i32, i32)> {
        Some(self.index.pop()?, self.velocity.pop()?)
    }

    pub fn iterate(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
        self.index.iter().zip(self.velocity.iter()).map(|&a, &b)| (a, b))
    }

    pub fn index_of(&self, index: u8) -> Option<i32> {
        self.index.iter().position(|&x| x == index)?
    }

    pub fn remove(&mut self, idx: i32) {
        self.index.remove(index);
        self.velocity.remove(index);
    }
}