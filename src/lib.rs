pub mod window;

#[cfg(test)]
mod tests {
    use crate::window::{
        create_window, Window
    };

    #[test]
    fn test_window() {
        let window : Window = Window{ width: 500, ..Default::default()};

        create_window(window);
    }
}
