use crate::pitch_class::PitchClass;

pub struct NoteSet {
    pub set: Vec<PitchClass>,
}

impl NoteSet {
    pub fn get_transposed_set(&self, num: i32) -> NoteSet {
        let mut new_set: Vec<PitchClass> = vec![];

        for i in self.set.iter() {
            let mut note = PitchClass { value: i.value };
            note.transpose(num);
            new_set.push(note);
        }

        return NoteSet { set: new_set };
    }

    pub fn get_set_transposed_to_0(&self) -> NoteSet {
        let amount = self.set[0].value;
        let mut new_set: Vec<PitchClass> = vec![];

        for i in self.set.iter() {
            let mut note = PitchClass { value: i.value };
            note.transpose(-amount);
            new_set.push(note);
        }

        return NoteSet { set: new_set };
    }

    //iterates over vec and prints Pitches
    pub fn print_set(&self) {
        for i in self.set.iter() {
            println!("{}", i);
        }
    }
}
