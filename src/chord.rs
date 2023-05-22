use crate::pitch_class::PitchClass;
use std::fmt;

#[derive(Debug)]
pub struct Chord {
    root: PitchClass,
    construction: ChordType,
    transformation_list: Vec<i32>,
    pitch_list: Vec<PitchClass>,
}

impl Chord {
    pub fn new(root: PitchClass, chord_type: ChordType) -> Chord {
        let notes = match_chord(&chord_type);

        return Chord {
            root: root.clone(),
            construction: chord_type,
            transformation_list: notes.clone(),
            pitch_list: create_note_list(notes, root),
        };
    }

    fn invert(&self) -> Chord {
        let new_vec = self.transformation_list.clone();
        let c = Chord {
            root: self.pitch_list[1].clone(),
            construction: ChordType::Custom(new_vec.clone()),
            transformation_list: new_vec.clone(),
            pitch_list: create_note_list(new_vec, self.pitch_list[1].clone()),
        };
        c
    }

    pub fn print_chord(&self) {
        println!("Root: {}", self.root);
        println!("Construction: {}", self.construction);
        for i in self.pitch_list.iter() {
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
