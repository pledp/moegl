use std::time::{Duration, Instant};
use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::graphics::GraphicsContext;
use crate::window::Window;
use crate::window::WinitPlugin;
use crate::app::PluginRegistry;
use crate::MoeglError;
use crate::app::Plugin;

pub enum GameState {
    Initializing,

    Running,

    QuitRequested,
}

/// Context for the application
pub struct Context {
    pub target_fps: u32,
    pub(crate) state: GameState,
    pub timer: Timer,

    pub(crate) plugin_registry: Option<PluginRegistry>,

    runner: Box<dyn FnOnce(Context) -> Result<(), MoeglError>>,
}

impl Context {
    /// Create context and init components
    pub(self) fn new(settings: &mut ContextBuilder) -> Result<Self, MoeglError> {
        let plugins = std::mem::take(&mut settings.plugins);
        let plugins_types = std::mem::take(&mut settings.plugins_types);
        let mut plugin_registry = PluginRegistry::from_vecs(plugins, plugins_types);

        for plugin in plugin_registry.plugins.iter_mut() {
            if let Some(plugin) = plugin.as_mut() {
                plugin.build(settings);
            }
        }

        Ok(Self {
            target_fps: settings.fps,
            state: GameState::Initializing,
            timer: Timer::new(),
            plugin_registry: Some(plugin_registry),
            runner: Box::new(run_once),
        })
    }

    pub fn get_plugin<P: Plugin + 'static>(&self) -> Option<&P> {
        let plugins = self.plugin_registry.as_ref().unwrap();
        plugins.get_plugin()
    }

    pub fn set_runner(&mut self, f: impl FnOnce(Context) -> Result<(), MoeglError> + 'static) {
        self.runner = Box::new(f);
    }

    pub(crate) fn frame_loop(&mut self, graphics_ctx: &mut GraphicsContext) {
        if self.timer.should_start_loop(self.target_fps) {

            let mut plugins = self.plugin_registry.take().unwrap();

            let len = plugins.plugins.len();
            for i in 0..len {
                // Temporarily take the plugin to avoid aliasing issues
                if let Some(mut plugin) = plugins.plugins[i].take() {
                    plugin.update(self, &mut plugins);
                    // Put it back after update
                    plugins.plugins[i] = Some(plugin);
                }
            }
            self.plugin_registry = Some(plugins);

            self.timer.stop_loop();
        }
    }

    pub fn set_fps(&mut self, target: u32) {
        self.target_fps = target;
    }

    pub fn set_gamestate(&mut self, state: GameState) {
        self.state = state;
    }

    pub fn run(mut self) -> Result<(), MoeglError>
    {
        let mut plugins = self.plugin_registry.take().unwrap();

        let len = plugins.plugins.len();
        for i in 0..len {
            // Temporarily take the plugin to avoid aliasing issues
            if let Some(mut plugin) = plugins.plugins[i].take() {
                plugin.init(&mut self);
                // Put it back after update
                plugins.plugins[i] = Some(plugin);
            }
        }
        self.plugin_registry = Some(plugins);

        self.set_gamestate(GameState::Running);

        let runner = core::mem::replace(&mut self.runner, Box::new(run_once));
        let app = self;
        println!("starting");

        (runner)(app)
    }
}

fn run_once(mut ctx: Context) -> Result<(), MoeglError> {
    Err(MoeglError::WinitError)
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

    pub(crate) plugins: Vec<Option<Box<dyn Plugin>>>,
    pub(crate) plugins_types: HashMap<TypeId, usize>,
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
        self.plugins.push(Some(Box::new(plugin)));
        self.plugins_types.insert(TypeId::of::<P>(), self.plugins.len() - 1);
        self
    }

    pub fn with_app<A: Plugin + 'static>(&mut self, app: A) -> &mut Self {
        self.plugins.push(Some(Box::new(app)));
        self.plugins_types.insert(TypeId::of::<A>(), self.plugins.len() - 1);
        self
    }

    /// Build context
    pub fn build(&mut self) -> Result<Context, MoeglError> {
        Context::new(self)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        let mut context_builder = ContextBuilder {
            title: "mogl".into(),
            width: 1280,
            height: 720,

            fps: 60,
            plugins: Vec::new(),
            plugins_types: HashMap::new(),
        };

        context_builder.with_plugin(WinitPlugin::default());

        context_builder
    }
}
