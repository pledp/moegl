use crate::window::Window;
use crate::App;
use crate::MoeglError;

pub enum GameState {
    Initializing,

    Running,

    QuitRequested,
}

/// Context for the application
pub struct Context {
    pub(crate) window: Window,
    pub(crate) state: GameState,

}

impl Context {
    /// Create context and init components
    pub(self) fn new(settings: &ContextBuilder) -> Result<Self, MoeglError> {
        let window = Window::new(settings);

        Ok(Self { 
            window,
            state: GameState::Initializing,
        })
    }

    pub fn update<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.update(self);
    }

    pub fn draw<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.draw(self);
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

        self.set_gamestate(GameState::Running);

        if let Err(e) = crate::window::run(self, app) {
            println!("{}", e);
        }
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
