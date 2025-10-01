use bevy_ecs::prelude::*;
use raylib::prelude::*;

#[derive(Resource, Clone)]  // << This is the key
pub struct Input {
    pub keys_down: Vec<KeyboardKey>,
    pub keys_pressed: Vec<KeyboardKey>,
    pub keys_released: Vec<KeyboardKey>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: vec![],
            keys_pressed: vec![],
            keys_released: vec![],
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.keys_down.clear();
        self.keys_pressed.clear();
        self.keys_released.clear();

        let tracked_keys = [
            KeyboardKey::KEY_W,
            KeyboardKey::KEY_A,
            KeyboardKey::KEY_S,
            KeyboardKey::KEY_D,
            KeyboardKey::KEY_SPACE,
            KeyboardKey::KEY_UP,
            KeyboardKey::KEY_DOWN,
            KeyboardKey::KEY_LEFT,
            KeyboardKey::KEY_RIGHT,
        
        
        ];

        for &key in &tracked_keys {
            if rl.is_key_down(key) {
                self.keys_down.push(key);
            }
            if rl.is_key_pressed(key) {
                self.keys_pressed.push(key);
            }
            if rl.is_key_released(key) {
                self.keys_released.push(key);
            }
        }
    }

    pub fn is_down(&self, key: KeyboardKey) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_pressed(&self, key: KeyboardKey) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_released(&self, key: KeyboardKey) -> bool {
        self.keys_released.contains(&key)
    }
}
