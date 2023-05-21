use music_test::note_set::NoteSet;
use music_test::pitch_class::PitchClass;
use music_test::NoteName;

#[test]
fn creation() {
    let d = PitchClass::new(NoteName::D);
    let a = PitchClass::new(NoteName::A);
    let d_again = PitchClass::new(NoteName::D);

    let new_set = NoteSet {
        set: vec![d, a, d_again],
    };
    let d2 = PitchClass::new(NoteName::D);
    let a2 = PitchClass::new(NoteName::A);
    let d2_again = PitchClass::new(NoteName::D);

    let new_set2 = NoteSet {
        set: vec![d2, a2, d2_again],
    };
    assert_eq!(new_set, new_set2,);
}
