use crate::event::System;

pub struct SystemStorage {
    system: Box<dyn System>,
}

impl SystemStorage {
    pub(crate) fn new(system: Box<dyn System>) -> Self {
        Self {
            system
        }
    }
}