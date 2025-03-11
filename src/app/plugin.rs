use std::any::{Any};
use downcast_rs::{Downcast, impl_downcast};

use crate::graphics::GraphicsContext;
use crate::app::{ Context, ContextBuilder, PluginRegistry };


pub trait Plugin: Downcast {
    fn build(&mut self, settings: &ContextBuilder) {}

    fn init(&mut self, ctx: &mut Context);

    fn update(&mut self, ctx: &mut Context, plugin_registry: &mut PluginRegistry) {}

    fn draw(&mut self, ctx: &mut Context, graphics_ctx: &mut GraphicsContext) {}
}

impl_downcast!(Plugin);