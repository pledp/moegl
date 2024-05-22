use crate::app::App;

#[derive(Debug)]
pub enum MoeglError {
    ContextError,

    WindowError,

    AppError,
}

/// Context for the application
pub struct Context {
    title: String,
    width: u32,
    height: u32,
    
    app: Box<dyn App>,
}

impl Context {
    pub(self) fn new(settings: ContextBuilder) -> Result<Self, MoeglError> {
        let app = settings.app.ok_or(MoeglError::AppError)?;

        Ok(Self {
            title: settings.title,
            width: settings.width,
            height: settings.height,

            app: app,
        })
    }

    pub fn run(&mut self) 
    {
        self.app.init();
        self.app.update();
        println!("Running!");
        println!("{}", self.title);
    }
}


/// Builder for context
pub struct ContextBuilder {
    title: String,
    width: u32,
    height: u32,

    app: Option<Box<dyn App>>,
}

impl ContextBuilder {
    pub fn new(title: &str, window_width: u32, window_height: u32) -> Self {
        Self {
            title: title.to_string(),
            width: window_width,
            height: window_height,
            
            ..Self::default()
        }
    }

    /// Add app to builder
    pub fn add_app<A>(mut self, app: A) -> Self 
    where
        A: App + 'static,
    {
        self.app = Some(Box::new(app));
        self
    }

    pub fn build(self) -> Result<Context, MoeglError> {
        Context::new(self)
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            title: "mogl".into(),
            width: 1280,
            height: 720,
            app: None,
        }
    }
}