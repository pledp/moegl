use crate::App;
use crate::MoeglError;
use crate::window::Window;

/// Context for the application
pub struct Context {
    window: Window,
    width: u32,
    height: u32,
}

impl Context {
    
    /// Create context and init components
    pub(self) fn new(settings: &ContextBuilder) -> Result<Self, MoeglError> {
        let window = Window::new(settings);

        Ok(Self {
            window,
            width: settings.width,
            height: settings.height,
        })
    }

    pub fn run<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.init();
        println!("Running!");

        // Run window
        if let Err(e) = self.window.run(self, app) {
            println!("{}", e);
        }
    }

    pub fn update<A>(&mut self, app: &A) 
    where
        A: App,
    {
        app.update();
    }

    pub fn draw<A>(&mut self, app: &A) 
    where 
        A: App,
    {
        app.draw();
    }
}

/// Builder for context
pub struct ContextBuilder {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl ContextBuilder {
    pub fn new<S>(title: S, window_width: u32, window_height: u32) -> Self 
    where
        S: Into<String>,
    {
        Self {
            title: title.into(),
            width: window_width,
            height: window_height,

            ..Self::default()
        }
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
        }
    }
}
