use godot::prelude::*;

#[derive(Default, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum Difficulty {
    Easy = 1,
    #[default]
    Balanced = 2,
    Hard = 3,
}

pub fn int_to_difficulty(difficulty: i32) -> Difficulty {
    match difficulty {
        1 => Difficulty::Easy,
        2 => Difficulty::Balanced,
        3 => Difficulty::Hard,
        _ => {
            godot_print!("Unknown difficulty {}", difficulty);
            // This is totally recoverable
            Difficulty::Hard
        }
    }
}
