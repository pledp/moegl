pub mod app;
mod error;
pub mod graphics;
pub mod input;
pub mod window;
mod texture;

pub use error::MoeglError;
pub use texture::Texture;

mod tests {
    use crate::{
        app::*,
        app::{Context, ContextBuilder},
    };

    #[test]
    fn plugin_test() {
        struct UserApp {}
        impl Plugin for UserApp {
            fn init(&mut self, ctx: &mut Context, plugin_registry: &mut PluginRegistry) {

            }
        
            fn update(&mut self, ctx: &mut Context, plugin_registry: &mut PluginRegistry) {
            }        
        }
        
        let result = ContextBuilder::new()
            .with_title("moegl test")
            .with_plugin(crate::input::InputPlugin::default())
            .with_app(UserApp {})
            .build();
    }
}