use std::time::{Duration, Instant};

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::window::Window;
use crate::App;
use crate::MoeglError;
use crate::graphics::GraphicsContext;


pub enum GameState {
    Initializing,

    Running,

    QuitRequested,
}

/// Context for the application
pub struct Context {
    pub(crate) window: Window,
    pub(crate) state: GameState,
    pub timer: Timer,
}

impl Context {
    /// Create context and init components
    pub(self) fn new(settings: &ContextBuilder) -> Result<Self, MoeglError> {
        let window = Window::new(settings);

        Ok(Self {
            window,
            state: GameState::Initializing,

            timer: Timer::new(),
        })
    }

    pub(crate) fn frame_loop<A>(&mut self, app: &A, graphics_context: &mut GraphicsContext)
    where
        A: App,
    {
        if self.timer.should_start_loop(self.window.fps) {
            self.update(app);
            self.draw(app, graphics_context);

            self.timer.stop_loop();
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.window.set_fps(fps);
    }

    pub fn set_gamestate(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn run<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.init(self);

        let event_loop = EventLoop::new().unwrap();

        let window = winit::window::WindowBuilder::new()
        .with_title(&self.window.title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            self.window.width,
            self.window.height,
        ))
        .build(&event_loop)
        .unwrap();

        let mut graphics_context = pollster::block_on(GraphicsContext::new(&window));

        self.set_gamestate(GameState::Running);

        if let Err(e) = crate::window::run(self, app, event_loop, graphics_context) {
            println!("{}", e);
        }
    }

    fn update<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.update(self);
    }

    fn draw<A>(&mut self, app: &A, graphics_context: &mut GraphicsContext)
    where
        A: App,
    {
        app.draw(self);

        match graphics_context.render() {
            Ok(_) => {}

            Err(wgpu::SurfaceError::Lost) => graphics_context.resize(graphics_context.size),

            Err(e) => eprint!("{:?}", e),
        }
    }
}

pub struct Timer {
    pub total_time: f64,
    pub delta_time: f64,

    last_frame_time: Instant,
}

impl Timer {
    fn new() -> Self {
        Timer {
            total_time: 0.0,
            delta_time: 0.0,
            last_frame_time: Instant::now(),
        }
    }

    pub(self) fn should_start_loop(&mut self, fps: u32) -> bool {
        let now = Instant::now();

        now - self.last_frame_time >= Duration::from_secs_f64(1.0 / fps as f64)
    }

    pub(self) fn stop_loop(&mut self) {
        let now = Instant::now();

        self.total_time += (now - self.last_frame_time).as_secs_f64();
        self.delta_time = (now - self.last_frame_time).as_secs_f64();

        self.last_frame_time = now;
    }
}

/// Builder for context
pub struct ContextBuilder {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn set_fps(&mut self, fps: u32) -> &mut Self {
        self.fps = fps;
        self
    }

    /// Build context
    pub fn build(&self) -> Result<Context, MoeglError> {
        Context::new(self)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            title: "mogl".into(),
            width: 1280,
            height: 720,

            fps: 60,
        }
    }
}
