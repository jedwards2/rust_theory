use music_test::chord::{Chord, ChordType};
use music_test::note_set::NoteSet;
use music_test::pitch_class::PitchClass;
use music_test::NoteName;

fn main() {
    let d = PitchClass::new(NoteName::D);
    let a = PitchClass::new(NoteName::A);
    let d2 = PitchClass::new(NoteName::D);

    let _bosh = NoteSet {
        set: vec![d, a, d2],
    };
    let d_new = PitchClass::new(NoteName::D);
    let size_vec = vec![2, 2, 2];
    let chord = Chord::new(d_new, ChordType::Custom(size_vec));
    chord.print_chord();
    // let new_set = bosh.get_transposed(0);

    // new_set.print_set();
    // let new_set2 = bosh.get_set_transposed_to_0();
    // bosh.print_set();
    // new_set2.print_set();
    // let singles = new_set.remove_duplicates();
    // singles.print_set();
}
