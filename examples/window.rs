use moegl::{app::*, context::{Context, ContextBuilder}};

fn main() {
    struct UserApp {}
    impl App for UserApp {
        fn init(&self, ctx: &mut Context) {
            ctx.set_fps(2);
            println!("init");
        }

        fn update(&self, ctx: &mut Context) {
            println!("update");
        }

        fn draw(&self, ctx: &mut Context) {
            println!("draw");
        }
    }
    
    let result = ContextBuilder::new("mogl test", 1280, 720).build();

    match result {
        Ok(mut context) => moegl::context::run(context, &UserApp{}),
        Err(e) => println!("Error: {}", e),
    }
}