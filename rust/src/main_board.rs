use crate::tetris::select::Select;
use godot::classes::{Label, Timer};
use godot::prelude::*;
use crate::constants::{COLOR_FOREGROUND, COLOR_SUCCESS};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct MainBoard {
    score: i64,
    high_score: i64,

    base: Base<Node2D>,
}

#[godot_api]
impl MainBoard {
    #[signal]
    fn global_game_over();

    #[func]
    fn start_game(&mut self) {
        self.base_mut().get_node_as::<Select>("Select0").show();
        self.base_mut().get_node_as::<Select>("Select1").show();
        self.base_mut().get_node_as::<Select>("Select2").show();
        self.base_mut().get_node_as::<Select>("Select3").show();
        self.base_mut().get_node_as::<Label>("LabelGameOver").hide();
        self.base_mut().get_node_as::<Node2D>("Score").show();
    }

    #[func]
    fn on_game_over(&mut self) {
        self.base_mut().get_node_as::<Label>("LabelGameOver").show();
        self.base_mut()
            .get_node_as::<Timer>("TimerGameOver")
            .start();
        self.base_mut().get_node_as::<Timer>("TimerScoreUpTimeout").stop();

        self.base_mut().emit_signal("global_game_over".into(), &[]);
    }

    #[func]
    fn on_game_over_timer_timeout(&mut self) {
        self.base_mut().get_node_as::<Node2D>("Score").hide();
        self.reset_score_color();
        self.score = 0;
    }

    #[func]
    fn on_score_up(&mut self) {
        self.score += 1;
        let mut score_label = self.base_mut()
            .get_node_as::<Label>("Score/LabelScore");
        score_label.set_text(format!("Score {}", self.score).into());
        score_label.set_modulate(COLOR_SUCCESS);

        if self.score > self.high_score {
            self.high_score = self.score;
            let mut high_score_label = self.base_mut()
                .get_node_as::<Label>("Score/LabelHigh");
            high_score_label.set_text(format!("High {}", self.score).into());
            high_score_label.set_modulate(COLOR_SUCCESS);
        }

        self.base_mut().get_node_as::<Timer>("TimerScoreUpTimeout").start();
    }

    #[func]
    fn reset_score_color(&mut self) {
        self.base_mut()
            .get_node_as::<Label>("Score/LabelScore").set_modulate(COLOR_FOREGROUND);
        self.base_mut()
            .get_node_as::<Label>("Score/LabelHigh").set_modulate(COLOR_FOREGROUND);
    }
}

#[godot_api]
impl INode2D for MainBoard {
    fn init(base: Base<Node2D>) -> Self {
        MainBoard {
            score: 0,
            high_score: 0,
            base,
        }
    }
}
