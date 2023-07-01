use serde::Deserialize;
use std::collections::HashMap;
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Unix,
    Windows,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Destination {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Unified(Destination),
    Platform(HashMap<Environment, Destination>),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Link {
    pub source: String,
    pub target: Target,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Linkfile {
    #[serde(rename = "link")]
    pub links: Vec<Link>,
}

#[cfg(test)]
mod tests {
    use super::{Destination, Environment, Link, Linkfile, Target};
    use std::collections::HashMap;

    #[test]
    fn smoke_linkfile() {
        const input: &str = r#"
            [[link]]
            source = "somefile"
            target.unix = "target_unix"
            target.windows = "target_windows"
        "#;

        let linkfile: Linkfile = toml::from_str(input).unwrap();

        assert_eq!(
            linkfile,
            Linkfile {
                links: vec![Link {
                    source: "somefile".to_string(),
                    target: Target::Platform(HashMap::from([
                        (
                            Environment::Unix,
                            Destination::Single("target_unix".to_string())
                        ),
                        (
                            Environment::Windows,
                            Destination::Single("target_windows".to_string())
                        )
                    ]))
                }]
            }
        );
    }
}
