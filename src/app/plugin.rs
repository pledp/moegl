use std::any::{Any};
use downcast_rs::{Downcast, impl_downcast};

use crate::app::Context;


pub trait Plugin: Downcast {
    fn init(&mut self, ctx: &mut Context);

    fn update(&mut self, ctx: &mut Context) {}
}

impl_downcast!(Plugin);