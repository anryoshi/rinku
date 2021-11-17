use std::{io, fmt};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Deserialization(toml::de::Error),
    Parse(&'static str),
    Misc(&'static str)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::Deserialization(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error occured: {:?}", err),
            Error::Deserialization(err) => write!(f, "Deserialization error occured: {:?}", err),
            Error::Parse(err) => write!(f, "Parse error occured: {:?}", err),
            Error::Misc(err) => write!(f,"Misc error occured: {:?}", err)
        }
    }
}
