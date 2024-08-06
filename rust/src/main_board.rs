use crate::constants::{COLOR_FOREGROUND, COLOR_SUCCESS};
use crate::tetris::select::Select;
use crate::ui::state::{get_difficulty, Difficulty};
use godot::classes::{InputEvent, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBoard {
    score: i64,
    high_score: i64,
    is_in_game_over: bool,

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

    #[func]
    fn start_game(&mut self) {
        self.score = 0;
        self.base()
            .get_node_as::<Label>("Score/LabelScore")
            .set_text("Score 0".into());

        self.base().get_node_as::<Select>("Select0").show();
        self.base().get_node_as::<Select>("Select1").show();
        self.base().get_node_as::<Select>("Select2").show();
        self.base().get_node_as::<Select>("Select3").show();
        self.base().get_node_as::<Node2D>("Score").show();
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
        if get_difficulty() >= Difficulty::Hard {
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
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.is_in_game_over && event.is_action_pressed("ui_accept".into()) {
            self.on_game_over_timer_timeout();
        }
    }
}
