mod keyboard;
mod mouse;

pub use keyboard::Keyboard;
pub use winit::keyboard::KeyCode;

use crate::app::{Plugin, Context};


pub struct InputPlugin {
    keyboard: Keyboard,
}

impl Plugin for InputPlugin {
    fn init(&mut self, ctx: &mut Context) {
        println!("init input plugin");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for InputPlugin {
    fn default() -> Self {
        InputPlugin {
            keyboard: Keyboard::new(),
        }
    }
}

