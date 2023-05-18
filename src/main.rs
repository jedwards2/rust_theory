use crate::pitch_class::PitchClass;
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
    let e = PitchClass::new(NoteName::C);
    // let a = PitchClass::new(NoteName::B);

    println!("VALUE: {}", e.value);
}
