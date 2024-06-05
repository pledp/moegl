use crate::context::Context;

pub trait App {
    fn init(&self, ctx: &mut Context);
    fn update(&self, ctx: &mut Context);
    fn draw(&self, ctx: &mut Context);
}
