use std::any::{TypeId};
use std::collections::HashMap;

use crate::app::Plugin;

pub struct PluginRegistry {
    pub plugins: Vec<Option<Box<dyn Plugin>>>,
    plugins_types: HashMap<TypeId, usize>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            plugins_types: HashMap::new(),
        }
    }

    pub fn from_vecs(plugins: Vec<Option<Box<dyn Plugin>>>, plugins_types: HashMap<TypeId, usize>) -> Self {
        Self {
            plugins,
            plugins_types
        }
    }

    pub fn get_plugin<P: Plugin + 'static>(&self) -> Option<&P> {
        let index = self.plugins_types
            .get(&TypeId::of::<P>()).unwrap().clone();

        self.plugins
            .get(index)
            .and_then(|plugin| plugin.as_ref()?.downcast_ref::<P>())
    }

    pub(crate) fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Some(Box::new(plugin)));
        self.plugins_types.insert(TypeId::of::<P>(), self.plugins.len() - 1);
    }
}