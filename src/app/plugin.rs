use std::any::{Any};

use crate::app::Context;


pub trait Plugin: Any {
    fn init(&mut self, ctx: &mut Context);

    fn update(&mut self, ctx: &mut Context) {}

    fn as_any(&self) -> &dyn Any;
}