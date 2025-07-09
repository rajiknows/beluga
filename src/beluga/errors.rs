#[derive(Debug)]
pub enum BelugaError {
    ProjectNotInitialised,
    IoError(std::io::Error),
}

impl std::fmt::Display for BelugaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BelugaError::ProjectNotInitialised => write!(f, "Project not initialised"),
            BelugaError::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl From<std::io::Error> for BelugaError {
    fn from(err: std::io::Error) -> Self {
        BelugaError::IoError(err)
    }
}
