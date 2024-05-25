use crate::app::App;
use crate::error::MoeglError;


/// Context for the application
pub struct Context {
    title: String,
    width: u32,
    height: u32,
}

impl Context {
    pub(self) fn new(settings: &ContextBuilder) -> Result<Self, MoeglError> {
        Ok(Self {
            title: settings.title.to_owned(),
            width: settings.width,
            height: settings.height,
        })
    }

    pub fn run<A>(&mut self, app: &A)
    where
        A: App,
    {
        app.init();
        app.update();
        println!("Running!");
        println!("{}", self.title);
    }
}

/// Builder for context
pub struct ContextBuilder {
    title: String,
    width: u32,
    height: u32,
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
