use crate::pitch_class::PitchClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoteSet {
    pub set: Vec<PitchClass>,
}

impl NoteSet {
    pub fn tranpose(&mut self, amt: i32) {
        for i in &mut self.set {
            i.transpose(amt);
        }
    }

    pub fn get_transposed(&self, amt: i32) -> NoteSet {
        let mut new_set: Vec<PitchClass> = vec![];

        for i in self.set.iter() {
            let mut note = PitchClass { value: i.value };
            note.transpose(amt);
            new_set.push(note);
        }

        return NoteSet { set: new_set };
    }

    pub fn transpose_to_0(&mut self) {
        let amt = self.set[0].value;
        for i in &mut self.set {
            i.transpose(-amt);
        }
    }

    pub fn get_transposed_to_0(&self) -> NoteSet {
        let amount = self.set[0].value;
        let mut new_set: Vec<PitchClass> = vec![];

        for i in self.set.iter() {
            let mut note = PitchClass { value: i.value };
            note.transpose(-amount);
            new_set.push(note);
        }

        return NoteSet { set: new_set };
    }

    pub fn remove_duplicates(&mut self) {
        let mut new_set: Vec<PitchClass> = vec![];
        for i in self.set.iter() {
            if !new_set.contains(i) {
                new_set.push((*i).clone());
            }
        }

        self.set = new_set;
    }

    pub fn get_duplicates_removed(&self) -> NoteSet {
        let mut new_set: Vec<PitchClass> = vec![];
        for i in self.set.iter() {
            if !new_set.contains(i) {
                new_set.push((*i).clone());
            }
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
