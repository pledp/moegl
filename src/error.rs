use std::fmt;

#[derive(Debug)]
pub enum MoeglError {
    ContextError,

    WindowError,

    AppError,

    WinitError,
}

impl fmt::Display for MoeglError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoeglError::ContextError => write!(f, "Context error"),
            MoeglError::WindowError => write!(f, "Error in creating window"),
            MoeglError::AppError => write!(f, "Invalid app"),
            MoeglError::WinitError => write!(f, "Winit error"),
        }
    }
}

impl std::error::Error for MoeglError {}