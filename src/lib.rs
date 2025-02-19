pub mod app;
mod error;
mod graphics;
pub mod input;
mod window;

pub use error::MoeglError;

mod tests {
    use crate::{
        app::*,
        app::{Context, ContextBuilder},
    };

    #[test]
    fn plugin_test() {
        struct UserApp {}
        impl Plugin for UserApp {
            fn init(&mut self, ctx: &mut Context) {

            }
        
            fn update(&mut self, ctx: &mut Context) {
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        
        }
        
        let result = ContextBuilder::new()
            .with_title("moegl test")
            .with_plugin(crate::input::InputPlugin::default())
            .with_app(UserApp {})
            .build();
    }
}