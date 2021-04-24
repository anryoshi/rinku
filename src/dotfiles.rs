use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use strum_macros;

#[derive(strum_macros::EnumString, strum_macros::ToString)]
pub enum Environment {
    #[strum(serialize = "unix")]
    UnixLike,
    #[strum(serialize = "windows")]
    Windows,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub enum Destination {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, untagged)]
pub enum Target {
    Unified(Destination),
    Platform(HashMap<String, Destination>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub source: String,
    pub target: Target,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dotfiles {
    #[serde(rename = "link")]
    pub links: Vec<Link>,
}
