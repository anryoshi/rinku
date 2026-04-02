use std::{fmt, io};
use strum;
use std::collections::HashMap;
use std::path;

#[derive(Debug)]
pub enum Error {
    BadLinkfilePath,
    BadLinkfile(io::Error),
    TomlParse(toml::de::Error),
    EnumParse(strum::ParseError),
    LinkfileContentError(Vec<(path::PathBuf, io::Error)>),
    TargetConflict(HashMap<path::PathBuf, Vec<path::PathBuf>>)
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
            Error::BadLinkfilePath => write!(f, "Path to linkfile is malformed\n"),
            Error::BadLinkfile(err) => write!(f, "IO error during linkfile processing: {:?}\n", err),
            Error::TomlParse(err) => write!(f, "TomlParse error occured:\n{}\n", err),
            Error::EnumParse(err) => write!(f, "EnumParse error occured: {:?}\n", err),
            Error::LinkfileContentError(errs) => {
                write!(f, "IO errors occured:\n")?;
                for (path, err) in errs {
                    write!(f, "{}:\t{}\n", path.display(), err)?;
                }
                Ok(())
            }
            Error::TargetConflict(err) => write!(f, "TargetConflict error occured: {:?}\n", err),
        }
    }
}
