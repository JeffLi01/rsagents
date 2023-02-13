use serde_json;

pub enum Error {
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerdeJson(arg0) => f.debug_tuple("SerdeJson").field(arg0).finish(),
            Self::Io(arg0) => f.debug_tuple("Io").field(arg0).finish(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
