use moegl::{app::*, context::ContextBuilder};

fn main() {
    struct UserApp {}
    impl App for UserApp {
        fn init(&self) {
            println!("init");
        }

        fn update(&self) {
            println!("update");
        }
    }
    
    let result = ContextBuilder::new("mogl test", 1280, 720).build();

    match result {
        Ok(mut context) => context.run(&UserApp{}),
        Err(e) => println!("Error: {}", e),
    }
}