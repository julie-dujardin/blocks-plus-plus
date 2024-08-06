use godot::prelude::*;

#[derive(Default, PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub enum Difficulty {
    Easy = 1,
    #[default]
    Balanced = 2,
    Hard = 3,
}

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct GameState {
    difficulty: Difficulty,

    base: Base<Object>,
}

#[godot_api]
impl GameState {
    #[func]
    fn set_difficulty_int(&mut self, difficulty: i32) {
        self.difficulty = match difficulty {
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
}

pub fn set_difficulty(difficulty: i32) {
    let mut state = godot::classes::Engine::singleton()
        .get_singleton(StringName::from("GameState"))
        .unwrap()
        .cast::<GameState>();
    let mut state_bind = state.bind_mut();
    state_bind.set_difficulty_int(difficulty);
}

pub fn get_difficulty() -> Difficulty {
    let mut state = godot::classes::Engine::singleton()
        .get_singleton(StringName::from("GameState"))
        .unwrap()
        .cast::<GameState>();
    let state_bind = state.bind_mut();
    state_bind.difficulty.clone()
}
