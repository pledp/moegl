// Create module, and re-export it
mod create_window;
pub use create_window::*;

pub struct Window {
    pub width: u32,
    pub height: u32,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            width: 320, 
            height: 240
        }
    }
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height
        }
    }
}