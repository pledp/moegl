pub mod app;
pub mod context;
mod error;
mod window;

pub use app::App;
pub use error::MoeglError;

#[cfg(test)]
mod tests {
    use crate::{app::*, context::ContextBuilder};

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

            fn draw(&self) {
                println!("draw");
            }
        }
        
        let result = ContextBuilder::new("mogl test", 1280, 720).build();

        match result {
            Ok(mut context) => context.run(&UserApp{}),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_app_2() {
        struct UserApp {}
        impl App for UserApp {
            fn init(&self) {
                println!("init");
            }

            fn update(&self) {
                println!("update");
            }

            fn draw(&self) {
                println!("draw");
            }
        }
        
        ContextBuilder::new("mogl test", 1280, 720)
            .build().unwrap()
            .run(&UserApp{});

    }
}
