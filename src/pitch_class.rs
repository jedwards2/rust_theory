use crate::NoteName;
use core::ops::Add;
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PitchClass {
    pub value: i32,
}

impl PitchClass {
    pub fn new(note: NoteName) -> PitchClass {
        PitchClass { value: note as i32 }
    }

    fn normalize(&mut self) {
        while self.value < 0 {
            self.value += 12;
        }
        self.value = self.value % 12
    }

    pub fn transpose(&mut self, num: i32) {
        self.value += num;
        self.normalize();
    }
}

impl Add for PitchClass {
    type Output = PitchClass;

    fn add(self, other: Self) -> Self::Output {
        let mut class = PitchClass {
            value: self.value + other.value,
        };

        class.normalize();
        class
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PitchClass: {}", self.value)
    }
}
