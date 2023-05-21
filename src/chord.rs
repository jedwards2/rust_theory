use crate::pitch_class::PitchClass;

pub struct Chord {
    root: PitchClass,
    construction: ChordType,
    note_list: Vec<PitchClass>,
}

pub enum ChordType {
    Major,
    Minor,
    Custom(Vec<i32>),
}

pub fn match_chord(c: &ChordType) -> Vec<i32> {
    match c {
        ChordType::Major => vec![0, 4, 7],
        ChordType::Minor => vec![0, 3, 7],
        ChordType::Custom(list) => list.to_vec(),
    }
}

pub fn create_note_list(num_list: Vec<i32>, root: PitchClass) -> Vec<PitchClass> {
    let mut pitch_list: Vec<PitchClass> = vec![root.clone()];
    let mut amount = num_list[0];
    for i in num_list.iter() {
        let mut new_root = root.clone();
        new_root.transpose(amount);
        pitch_list.push(new_root);
        amount += num_list[*i as usize]
    }
    pitch_list
}

impl Chord {
    fn new(root: PitchClass, chord_type: ChordType) -> Chord {
        let notes = match_chord(&chord_type);

        return Chord {
            root: root.clone(),
            construction: chord_type,
            note_list: create_note_list(notes, root),
        };
    }

    fn print_chord() {}
}
