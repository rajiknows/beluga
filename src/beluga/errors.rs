#[derive(Debug)]
pub enum BelugaError {
    ProjectNotInitialised,
}

impl std::fmt::Display for BelugaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BelugaError::ProjectNotInitialised => write!(f, "Project not initialised"),
        }
    }
}
