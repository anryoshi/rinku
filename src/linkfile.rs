use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(PartialEq, Eq, Hash, Debug, Deserialize, Serialize, Display, EnumString)]
pub enum Environment {
    #[strum(serialize = "unix")]
    #[serde(rename = "unix")]
    Unix,
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
pub struct Linkfile {
    #[serde(rename = "link")]
    pub links: Vec<Link>,
}
