use crate::note_set::NoteSet;
use crate::pitch_class::PitchClass;
use std::fmt;

#[derive(Debug)]
pub struct Chord {
    pub pitch_list: NoteSet,
}

impl Chord {
    pub fn invert(&self) -> Chord {
        let mut new_set = self.pitch_list.set.clone();
        let first = new_set.first().unwrap();
        new_set.push(first.clone());
        new_set.remove(0);
        let final_set = NoteSet { set: new_set };
        let chord = Chord {
            pitch_list: final_set,
        };
        chord
    }

    pub fn print_chord(&self) {
        for i in self.pitch_list.set.iter() {
            i.pretty_print();
        }
    }
}

//custom should be a Vec<i32> of distances in half steps between notes
#[derive(Debug)]
pub enum ChordType {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7th,
    Custom(Vec<i32>),
}

impl fmt::Display for ChordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn match_chord(c: &ChordType) -> Vec<i32> {
    match c {
        ChordType::Major => vec![4, 3],
        ChordType::Minor => vec![3, 4],
        ChordType::Diminished => vec![3, 3],
        ChordType::Augmented => vec![4, 4],
        ChordType::Major7th => vec![4, 3, 3],
        ChordType::Custom(list) => list.to_vec(),
    }
}

pub fn create_note_list(num_list: Vec<i32>, root: PitchClass) -> Vec<PitchClass> {
    let mut pitch_list: Vec<PitchClass> = vec![root.clone()];
    let mut amount = 0;
    for i in num_list.iter() {
        amount += *i;
        let mut new_root = root.clone();
        new_root.transpose(amount);
        pitch_list.push(new_root);
    }
    pitch_list
}
