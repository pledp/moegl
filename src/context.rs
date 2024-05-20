use crate::app::App;

pub struct Context {
    title: String,
    width: u32,
    height: u32,    
}

impl Context {
    pub(self) fn new(settings: ContextBuilder) -> Result<Self, &'static str> {
        Ok(Self {
            title: settings.title,
            width: settings.width,
            height: settings.height,
        })
    }

    pub fn run<T>(&mut self, app: T) 
    where
        T: App,
    {
        app.init();
        app.update();
        println!("Running!");
    }
}


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

    pub fn build(self) -> Result<Context, &'static str> {
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