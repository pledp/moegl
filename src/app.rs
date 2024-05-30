pub trait App {
    fn init(&self);
    fn update(&self);
    fn draw(&self);
}
