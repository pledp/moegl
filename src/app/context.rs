use std::time::{Duration, Instant};
use std::sync::Arc;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    event_loop::EventLoopBuilder,
};

use crate::graphics::GraphicsContext;
use crate::input::Keyboard;
use crate::window::Window;
use crate::MoeglError;
use crate::app::Plugin;

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

    pub(crate) event_loop: Option<EventLoop<()>>,
    pub graphics_context: GraphicsContext,

    plugins: Option<Vec<Box<dyn Plugin>>>,
}

impl Context {
    /// Create context and init components
    pub(self) fn new(settings: &mut ContextBuilder) -> Result<Self, MoeglError> {
        /// TODO: Seperate wgpu and winit things completely from Context
        let window = Window::new(settings);

        let mut event_loop_builder = EventLoopBuilder::new();
        
        /// TODO: Create winit plugin
        #[cfg(target_os= "windows")]
        {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            event_loop_builder.with_any_thread(true);
        }

        let event_loop = event_loop_builder.build().unwrap();

        let winit_window = winit::window::WindowBuilder::new()
            .with_title(&settings.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                settings.width,
                settings.height,
            ))
            .build(&event_loop)
            .unwrap();

        let mut graphics_context = pollster::block_on(GraphicsContext::new(winit_window));

        let plugins = std::mem::take(&mut settings.plugins);
        
        Ok(Self {
            window,
            state: GameState::Initializing,

            timer: Timer::new(),

            event_loop: Some(event_loop),
            graphics_context,

            plugins: Some(plugins),
        })
    }

    pub(crate) fn frame_loop(&mut self) {
        if self.timer.should_start_loop(self.window.fps) {

            let mut plugins = self.plugins.take().unwrap();

            for plugin in plugins.iter_mut() {
                plugin.update(self);
            }
    
            self.plugins = Some(plugins);

            self.timer.stop_loop();
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.window.set_fps(fps);
    }

    pub fn set_gamestate(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn run(&mut self)
    {
        let mut plugins = self.plugins.take().unwrap();

        for plugin in plugins.iter_mut() {
            plugin.init(self);
        }

        self.plugins = Some(plugins);

        self.set_gamestate(GameState::Running);

        if let Err(e) = crate::window::run(self) {
            println!("{}", e);
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
    pub(crate) title: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) fps: u32,

    pub(crate) plugins: Vec<Box<dyn Plugin>>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    pub fn with_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    pub fn with_title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn with_fps(&mut self, fps: u32) -> &mut Self {
        self.fps = fps;
        self
    }

    pub fn with_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn with_app<A: Plugin + 'static>(&mut self, app: A) -> &mut Self {
        self.plugins.push(Box::new(app));
        self
    }

    /// Build context
    pub fn build(&mut self) -> Result<Context, MoeglError> {
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
            plugins: Vec::new(),
        }
    }
}
