use crate::app::Context;

pub trait Plugin {
    fn init(&mut self, ctx: &mut Context);

    fn update(&mut self, ctx: &mut Context) {}
}