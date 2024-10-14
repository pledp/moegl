use moegl::{
    app::*,
    context::{Context, ContextBuilder},
};


struct UserApp {}
impl App for UserApp {
    fn init(&self, ctx: &mut Context) {
        ctx.set_fps(2);
        println!("init");
    }

    fn update(&self, ctx: &mut Context) {
        println!("{}", ctx.timer.delta_time);
        println!("update");
    }

    fn draw(&self, ctx: &mut Context) {
        println!("draw");
    }
}

fn main() {
    env_logger::init();
    let result = ContextBuilder::new().set_title("moegl test").build();

    match result {
        Ok(mut context) => context.run(&UserApp {}),
        Err(e) => println!("Error: {}", e),
    }

    println!("Finishing up");
}
