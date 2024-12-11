use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Error parsing input: {0}")]
    Parse(String, #[source] Option<Box<dyn std::error::Error>>),

    #[error("Solution not found: {0}")]
    SolutionNotFound(String),
}

pub fn parse_error(msg: &str, line: &str) -> Error {
    Error::Parse(format!("error parsing '{}': {}", line, msg), None)
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::Parse("error parsing integer".to_string(), Some(Box::new(value)))
    }
}
