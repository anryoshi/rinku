use std::{io, fmt};
use strum;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TomlParse(toml::de::Error),
    EnumParse(strum::ParseError),
    Misc(&'static str)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::TomlParse(e)
    }
}

impl From<strum::ParseError> for Error {
    fn from(e: strum::ParseError) -> Self {
        Error::EnumParse(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error occured: {:?}", err),
            Error::TomlParse(err) => write!(f, "TomlParse error occured: {:?}", err),
            Error::EnumParse(err) => write!(f, "EnumParse error occured: {:?}", err),
            Error::Misc(err) => write!(f,"Misc error occured: {:?}", err)
        }
    }
}
