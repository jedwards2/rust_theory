use music_test::pitch_class::PitchClass;
use music_test::NoteName;

#[test]
fn tranpose() {
    let mut d = PitchClass::new(NoteName::D);
    d.transpose(5);

    let g = PitchClass::new(NoteName::G);
    assert_eq!(d, g);
}

#[test]
fn tranpose_above_octave() {
    let mut b = PitchClass::new(NoteName::B);
    b.transpose(1);
    let c = PitchClass::new(NoteName::C);
    assert_eq!(b, c);
}
