use crate::note_set::NoteSet;
use crate::pitch_class::PitchClass;
mod note_set;
mod pitch_class;

pub enum NoteName {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

fn main() {
    let d = PitchClass::new(NoteName::D);
    let a = PitchClass::new(NoteName::A);

    let bosh = NoteSet { set: vec![d, a] };

    let new_set = bosh.get_transposed_set(4);

    new_set.print_set();
    let new_set2 = bosh.get_set_transposed_to_0();
    bosh.print_set();
    new_set2.print_set();
}
