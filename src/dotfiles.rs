use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display};

#[derive(PartialEq, Eq, Hash, Debug, Deserialize, Serialize, Display)]
pub enum Environment {
    #[strum(serialize = "unix")]
    #[serde(rename = "unix")]
    UnixLike,
    #[strum(serialize = "windows")]
    #[serde(rename = "windows")]
    Windows,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Destination {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Target {
    Unified(Destination),
    Platform(HashMap<Environment, Destination>),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    pub source: String,
    pub target: Target,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Dotfiles {
    #[serde(rename = "link")]
    pub links: Vec<Link>,
}
