// Keyboard input implementation
use std::collections::HashSet;

use winit::{
    event::*,
    keyboard::KeyCode,
};

pub struct Keyboard {
    pressed_keys: HashSet<KeyCode>,
    timestep_pressed_keys: HashSet<KeyCode>,
}

impl Keyboard {
    pub(crate) fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            timestep_pressed_keys: HashSet::new(),
        }
    }

    pub(crate) fn handle_input(&mut self, event: &KeyEvent, code: KeyCode) {
        if !event.repeat {
            match event.state {
                ElementState::Pressed => {
                    self.pressed_keys.insert(code);
                }

                ElementState::Released => {
                    self.pressed_keys.remove(&code);
                }
            }
        }

        self.timestep_pressed_keys.insert(code);
    }

    /// Check if key is currently pressed, regardless of timestep.
    /// If planned to use with timesteps, check out:
    /// - [`Self::timestep_is_pressed()`]
    /// - [`Self::reset_timestep()`]
    pub fn is_pressed(&self, code: &KeyCode) -> bool {
        self.pressed_keys.contains(code)
    }

    pub fn reset_timestep(&mut self) {
        self.timestep_pressed_keys = HashSet::new();
    }

    /// Check if key was pressed in current timestep.
    /// If the current key state is desired, check out:
    /// - [`Self::is_pressed()`]
    pub fn timestep_is_pressed(&self, code: &KeyCode) -> bool {
        self.timestep_pressed_keys.contains(code)
    }
}
