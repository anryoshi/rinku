use std::{io, fmt};

#[derive(Debug)]
pub enum LinkerError {
    Io(io::Error),
    Parse(toml::de::Error),
    Misc(String)
}

impl From<io::Error> for LinkerError {
    fn from(e: io::Error) -> Self {
        LinkerError::Io(e)
    }
}

impl fmt::Display for LinkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkerError::Io(err) => write!(f, "IO error occured: {:?}", err),
            LinkerError::Parse(err) => write!(f, "Parse error occured: {:?}", err),
            LinkerError::Misc(err) => write!(f,"Misc error occured: {:?}", err)
        }
    }
}
