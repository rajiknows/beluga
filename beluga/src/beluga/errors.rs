#[derive(Debug)]
pub enum BelugaError {
    ProjectNotInitialised,
    IoError(std::io::Error),
    RhaiError(Box<dyn std::error::Error>),
    Other(String),
}

impl std::fmt::Display for BelugaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BelugaError::ProjectNotInitialised => write!(f, "Project not initialised"),
            BelugaError::IoError(e) => write!(f, "IO Error: {}", e),
            BelugaError::RhaiError(e) => write!(f, "Rhai Error: {}", e),
            Self::Other(error_message) => write!(f, "error: {}", error_message),
        }
    }
}

impl From<std::io::Error> for BelugaError {
    fn from(err: std::io::Error) -> Self {
        BelugaError::IoError(err)
    }
}

impl From<Box<dyn std::error::Error>> for BelugaError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        BelugaError::RhaiError(err)
    }
}