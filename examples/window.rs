use moegl::{
    app::*,
    context::{Context, ContextBuilder},
    input::KeyCode,
    event::{System, ResizeEvent}
};

struct UserApp {}
impl App for UserApp {
    fn init(&self, ctx: &mut Context) {
        ctx.set_fps(2);
        println!("init");
    }

    fn update(&self, ctx: &mut Context) {
        println!("{},", ctx.keyboard.timestep_is_pressed(&KeyCode::KeyA));
    }

    fn draw(&self, ctx: &mut Context) {}
}

impl System for UserApp {
    
}

fn handle_resize(system: &mut UserApp, event: &ResizeEvent) {

}

fn main() {
    env_logger::init();
    let app = UserApp {};

    let result = ContextBuilder::new()
    .with_title("moegl test")
    .with_app(app)
    .build();

    match result {
        Ok(mut context) => context.run(&UserApp {}),
        Err(e) => println!("Error: {}", e),
    }

    println!("Finishing up");
}
