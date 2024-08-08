use crate::constants::{COLOR_FOREGROUND, COLOR_SUCCESS};
use crate::ui::state::{int_to_difficulty, Difficulty};
use godot::classes::{InputEvent, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBoard {
    score: i64,
    high_score: i64,
    is_in_game_over: bool,
    difficulty: Difficulty,

    base: Base<Node2D>,
}

#[godot_api]
impl MainBoard {
    #[signal]
    fn global_game_over();

    #[signal]
    fn global_game_over_timeout();

    #[signal]
    fn global_score_timed_out();

    #[signal]
    fn global_game_init();

    #[func]
    fn start_game(&mut self, difficulty: Variant) {
        self.difficulty = int_to_difficulty(difficulty.to::<i32>());

        self.score = 0;
        self.base()
            .get_node_as::<Label>("Score/LabelScore")
            .set_text("Score 0".into());
        self.base().get_node_as::<Node2D>("Score").show();

        self.base_mut()
            .emit_signal("global_game_init".into(), &[difficulty]);
    }

    #[func]
    fn on_game_over(&mut self) {
        self.base().get_node_as::<Label>("LabelGameOver").show();
        self.base().get_node_as::<Timer>("TimerGameOver").start();
        self.base()
            .get_node_as::<Timer>("TimerScoreUpTimeout")
            .stop();

        self.base_mut().emit_signal("global_game_over".into(), &[]);
        self.is_in_game_over = true;
    }

    #[func]
    fn on_game_over_timer_timeout(&mut self) {
        if self.is_in_game_over {
            self.is_in_game_over = false;
            self.reset_score_color();
            self.base().get_node_as::<Label>("LabelGameOver").hide();
            self.base_mut()
                .emit_signal("global_game_over_timeout".into(), &[]);
        }
    }

    #[func]
    fn on_score_up(&mut self, count: Variant) {
        self.score += count.to::<i64>();
        let mut score_label = self.base().get_node_as::<Label>("Score/LabelScore");
        score_label.set_text(format!("Score {}", self.score).into());
        score_label.set_modulate(COLOR_SUCCESS);

        if self.score > self.high_score {
            self.high_score = self.score;
            let mut high_score_label = self.base().get_node_as::<Label>("Score/LabelHigh");
            high_score_label.set_text(format!("High {}", self.score).into());
            high_score_label.set_modulate(COLOR_SUCCESS);
        }

        self.base()
            .get_node_as::<Timer>("TimerScoreUpTimeout")
            .start();
    }

    #[func]
    fn on_score_timed_out(&mut self) {
        if self.difficulty >= Difficulty::Hard {
            self.base_mut()
                .emit_signal("global_score_timed_out".into(), &[]);
        }
    }

    #[func]
    fn reset_score_color(&mut self) {
        self.base()
            .get_node_as::<Label>("Score/LabelScore")
            .set_modulate(COLOR_FOREGROUND);
        self.base()
            .get_node_as::<Label>("Score/LabelHigh")
            .set_modulate(COLOR_FOREGROUND);
    }
}

#[godot_api]
impl INode2D for MainBoard {
    fn init(base: Base<Node2D>) -> Self {
        MainBoard {
            score: 0,
            high_score: 0,
            is_in_game_over: false,
            difficulty: Difficulty::default(),
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.is_in_game_over && event.is_action_pressed("ui_accept".into()) {
            self.on_game_over_timer_timeout();
        }
    }
}
