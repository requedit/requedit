use std::fmt;


#[derive(Debug)]
pub(crate) enum RequeditError {
    Hudsucker(hudsucker::Error),
    Hyper(hyper::Error),
    IO(std::io::Error),
    SerdeYaml(serde_yaml::Error),
    SerdeJson(serde_json::Error),
    Utf8Error(std::str::Utf8Error),
    Other(String),
}

impl From<hudsucker::Error> for RequeditError {
    fn from(err: hudsucker::Error) -> Self {
        RequeditError::Hudsucker(err)
    }
}

impl From<hyper::Error> for RequeditError {
    fn from(err: hyper::Error) -> Self {
        RequeditError::Hyper(err)
    }
}
impl From<std::io::Error> for RequeditError {
    fn from(err: std::io::Error) -> Self {
        RequeditError::IO(err)
    }
}
impl From<serde_yaml::Error> for RequeditError {
    fn from(err: serde_yaml::Error) -> Self {
        RequeditError::SerdeYaml(err)
    }
}

impl From<serde_json::Error> for RequeditError {
    fn from(err: serde_json::Error) -> Self {
        RequeditError::SerdeJson(err)
    }
}

impl From<std::str::Utf8Error> for RequeditError {
    fn from(value: std::str::Utf8Error) -> Self {
        RequeditError::Utf8Error(value)
    }
}

impl fmt::Display for RequeditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequeditError::Hyper(e) => write!(f, "Hyper error: {}", e),
            RequeditError::Hudsucker(e) => write!(f, "Hudsucker error: {}", e),
            RequeditError::IO(e) => write!(f, "IO error: {}", e),
            RequeditError::SerdeYaml(e) => write!(f, "SerdeYaml error: {}", e),
            RequeditError::SerdeJson(e) => write!(f, "SerdeJson error: {}", e),
            RequeditError::Utf8Error(e) => write!(f, "Utf8 error: {}", e),
            RequeditError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}
impl std::error::Error for RequeditError {}
