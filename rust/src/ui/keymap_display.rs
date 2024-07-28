use godot::classes::{Control, IControl, InputEvent};
use godot::prelude::*;
use once_cell::unsync::Lazy;
use std::collections::HashMap;

// Keymap generated by chatGPT 4
const KEYMAP: Lazy<[Vec<(&str, f32, f32, f32)>; 6]> = Lazy::new(|| {
    [
        // Function row
        vec![
            ("ESC", 0.0, 1.0, 1.0),
            ("F1", 2.0, 1.0, 1.0),
            ("F2", 3.0, 1.0, 1.0),
            ("F3", 4.0, 1.0, 1.0),
            ("F4", 5.0, 1.0, 1.0),
            ("F5", 6.5, 1.0, 1.0),
            ("F6", 7.5, 1.0, 1.0),
            ("F7", 8.5, 1.0, 1.0),
            ("F8", 9.5, 1.0, 1.0),
            ("F9", 11.0, 1.0, 1.0),
            ("F10", 12.0, 1.0, 1.0),
            ("F11", 13.0, 1.0, 1.0),
            ("F12", 14.0, 1.0, 1.0),
            ("PRINT_SCREEN", 15.5, 1.0, 1.0),
            ("SCROLL_LOCK", 16.5, 1.0, 1.0),
            ("PAUSE", 17.5, 1.0, 1.0),
        ],
        // Numbers row, first row of nav cluster, first row of numpad
        vec![
            ("BACKQUOTE", 0.0, 1.0, 1.0),
            ("1", 1.0, 1.0, 1.0),
            ("2", 2.0, 1.0, 1.0),
            ("3", 3.0, 1.0, 1.0),
            ("4", 4.0, 1.0, 1.0),
            ("5", 5.0, 1.0, 1.0),
            ("6", 6.0, 1.0, 1.0),
            ("7", 7.0, 1.0, 1.0),
            ("8", 8.0, 1.0, 1.0),
            ("9", 9.0, 1.0, 1.0),
            ("0", 10.0, 1.0, 1.0),
            ("MINUS", 11.0, 1.0, 1.0),
            ("EQUAL", 12.0, 1.0, 1.0),
            ("BACKSPACE", 13.0, 2.0, 1.0),
            ("INSERT", 15.5, 1.0, 1.0),
            ("HOME", 16.5, 1.0, 1.0),
            ("PAGE_UP", 17.5, 1.0, 1.0),
            ("NUM_LOCK", 19.0, 1.0, 1.0),
            ("NUM_SLASH", 20.0, 1.0, 1.0),
            ("NUM_ASTERISK", 21.0, 1.0, 1.0),
            ("NUM_MINUS", 22.0, 1.0, 1.0),
        ],
        // Tab row, second row of nav cluster, second row of numpad
        vec![
            ("TAB", 0.0, 1.5, 1.0),
            ("Q", 1.5, 1.0, 1.0),
            ("W", 2.5, 1.0, 1.0),
            ("E", 3.5, 1.0, 1.0),
            ("R", 4.5, 1.0, 1.0),
            ("T", 5.5, 1.0, 1.0),
            ("Y", 6.5, 1.0, 1.0),
            ("U", 7.5, 1.0, 1.0),
            ("I", 8.5, 1.0, 1.0),
            ("O", 9.5, 1.0, 1.0),
            ("P", 10.5, 1.0, 1.0),
            ("LEFT_BRACKET", 11.5, 1.0, 1.0),
            ("RIGHT_BRACKET", 12.5, 1.0, 1.0),
            ("BACKSLASH", 13.5, 1.5, 1.0),
            ("DELETE", 15.5, 1.0, 1.0),
            ("END", 16.5, 1.0, 1.0),
            ("PAGE_DOWN", 17.5, 1.0, 1.0),
            ("NUM_7", 19.0, 1.0, 1.0),
            ("NUM_8", 20.0, 1.0, 1.0),
            ("NUM_9", 21.0, 1.0, 1.0),
            ("NUM_PLUS", 22.0, 1.0, 2.0),
        ],
        // Caps Lock row, third row of numpad
        vec![
            ("CAPS_LOCK", 0.0, 1.75, 1.0),
            ("A", 1.75, 1.0, 1.0),
            ("S", 2.75, 1.0, 1.0),
            ("D", 3.75, 1.0, 1.0),
            ("F", 4.75, 1.0, 1.0),
            ("G", 5.75, 1.0, 1.0),
            ("H", 6.75, 1.0, 1.0),
            ("J", 7.75, 1.0, 1.0),
            ("K", 8.75, 1.0, 1.0),
            ("L", 9.75, 1.0, 1.0),
            ("SEMICOLON", 10.75, 1.0, 1.0),
            ("QUOTE", 11.75, 1.0, 1.0),
            ("ENTER", 12.75, 2.25, 1.0),
            ("NUM_4", 19.0, 1.0, 1.0),
            ("NUM_5", 20.0, 1.0, 1.0),
            ("NUM_6", 21.0, 1.0, 1.0),
        ],
        // Shift row, fourth row of numpad
        vec![
            ("LEFT_SHIFT", 0.0, 2.25, 1.0),
            ("Z", 2.25, 1.0, 1.0),
            ("X", 3.25, 1.0, 1.0),
            ("C", 4.25, 1.0, 1.0),
            ("V", 5.25, 1.0, 1.0),
            ("B", 6.25, 1.0, 1.0),
            ("N", 7.25, 1.0, 1.0),
            ("M", 8.25, 1.0, 1.0),
            ("COMMA", 9.25, 1.0, 1.0),
            ("PERIOD", 10.25, 1.0, 1.0),
            ("SLASH", 11.25, 1.0, 1.0),
            ("RIGHT_SHIFT", 12.25, 2.75, 1.0),
            ("UP_ARROW", 16.5, 1.0, 1.0),
            ("NUM_1", 19.0, 1.0, 1.0),
            ("NUM_2", 20.0, 1.0, 1.0),
            ("NUM_3", 21.0, 1.0, 1.0),
            ("NUM_ENTER", 22.0, 1.0, 2.0),
        ],
        // Control row, last row of numpad
        vec![
            ("LEFT_CTRL", 0.0, 1.25, 1.0),
            ("LEFT_WIN", 1.25, 1.25, 1.0),
            ("LEFT_ALT", 2.5, 1.25, 1.0),
            ("SPACE", 3.75, 6.25, 1.0),
            ("RIGHT_ALT", 10., 1.25, 1.0),
            ("RIGHT_WIN", 11.25, 1.25, 1.0),
            ("MENU", 12.5, 1.25, 1.0),
            ("RIGHT_CTRL", 13.75, 1.25, 1.0),
            ("LEFT_ARROW", 15.5, 1.0, 1.0),
            ("DOWN_ARROW", 16.5, 1.0, 1.0),
            ("RIGHT_ARROW", 17.5, 1.0, 1.0),
            ("NUM_0", 19.0, 2.0, 1.0),
            ("NUM_PERIOD", 21.0, 1.0, 1.0),
        ],
    ]
});

#[derive(GodotClass)]
#[class(base=Control)]
pub struct KeymapDisplay {
    keys: HashMap<&'static str, Gd<Control>>,

    base: Base<Control>,
}

#[godot_api]
impl IControl for KeymapDisplay {
    fn init(base: Base<Control>) -> Self {
        KeymapDisplay {
            keys: HashMap::new(),
            base,
        }
    }

    fn ready(&mut self) {
        let key_scene: Gd<PackedScene> = load("res://scenes/ui/key.tscn");

        for (y, line) in KEYMAP.iter().enumerate() {
            for (key_name, key_offset, key_width, key_height) in line.iter() {
                let mut key_instance = key_scene.instantiate_as::<Control>();
                let key_size = key_instance.get_size();
                let key_size_with_border = key_size;
                key_instance.set_position(Vector2::new(
                    key_size_with_border.x * key_offset,
                    key_size_with_border.y * y as f32,
                ));
                key_instance.set_size(key_size * Vector2::new(*key_width, *key_height));

                self.base_mut().add_child(key_instance.clone().upcast());
                self.keys.insert(key_name, key_instance);
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("down".into()) {
            // TODO
        } else if event.is_action_pressed("up".into()) {
            // TODO
        } else if event.is_action_pressed("left".into()) {
            // TODO
        } else if event.is_action_pressed("right".into()) {
            // TODO
        }
    }
}
