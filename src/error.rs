use std::fmt;

#[derive(Debug)]
pub enum MoeglError {
    ContextError,
    WindowError,
    AppError,
    WinitError,
    Asset,
}

impl fmt::Display for MoeglError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MoeglError::ContextError => write!(f, "Context error"),
            MoeglError::WindowError => write!(f, "Error in creating window"),
            MoeglError::AppError => write!(f, "App error"),
            MoeglError::WinitError => write!(f, "Winit error"),
            MoeglError::Asset => write!(f, "Asset error"),
        }
    }
}

impl std::error::Error for MoeglError {}

// TODO: Create external error for different systems: No monolithic error struct
impl From<image::ImageError> for MoeglError {
    fn from(err: image::ImageError) -> Self {
        MoeglError::Asset
    }
}

impl From<std::io::Error> for MoeglError {
    fn from(err: std::io::Error) -> Self {
        MoeglError::Asset
    }
}

