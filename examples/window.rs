use moegl::{
    app::*,
    input::KeyCode,
};

struct UserApp {}
impl Plugin for UserApp {
    fn init(&mut self, ctx: &mut Context) {
        ctx.set_fps(2);
        println!("init");
    }

    fn update(&mut self, ctx: &mut Context) {
        println!("update");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn main() {
    env_logger::init();
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
