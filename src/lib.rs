pub mod window;
pub mod app;
pub mod context;

#[cfg(test)]
mod tests {
    use crate::{
        context::ContextBuilder,
        app::*,
    };

    #[test]
    fn test_app() {
        struct UserApp {}
        impl App for UserApp {
            fn init(&self) {
                println!("init");
            }

            fn update(&self) {
                println!("update");
            }
        }

        ContextBuilder::new("mogl test", 1280, 720)
            .build().expect("Failed")
            .run(UserApp{});
    }
}
