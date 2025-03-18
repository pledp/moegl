use moegl::{
    app::*,
    input::KeyCode,
    graphics::GraphicsContext,
    window::WinitPlugin,
};

struct UserApp {}
impl Plugin for UserApp {
    fn init(&mut self, ctx: &mut Context) {
        ctx.set_fps(2);
        println!("init");
    }

    fn update(&mut self, ctx: &mut Context, plugin_registry: &mut PluginRegistry) {
        let window = plugin_registry.get_plugin::<WinitPlugin>().unwrap();
        println!("{}", window.window.title);

        println!("update");
    }

    fn draw(&mut self, ctx: &mut Context, graphics_ctx: &mut GraphicsContext) {

    }
}

fn main() {
    let app = UserApp {};

    let result = ContextBuilder::new()
        .with_title("moegl test")
        .with_app(app)
        .build();

    match result {
        Ok(mut context) => {
            context.run();
            ()
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("Finishing up");
}
