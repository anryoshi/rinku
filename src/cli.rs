pub use clap::Parser;

use clap::ValueEnum;
use std::path;

#[derive(Copy, Clone, ValueEnum)]
pub enum Mode {
    /// Only prints status of the targets
    Dry,

    /// Performs linking only when no single target exists
    Strict,

    /// Fills missing targets, ignores existing one
    Lazy,

    /// Overrides all targets with copyng old versions to the `*.orig.<#>`
    Force,
}

#[derive(Parser)]
pub struct Cli {
    /// e.g. dotfiles.toml
    pub linkfile: path::PathBuf,

    /// Operation mode
    #[arg(
        value_enum,
        short = 'm',
        long = "mode",
        default_value_t = Mode::Dry
    )]
    pub mode: Mode,

    /// Explicitly specified tags
    #[arg(
        short = 't',
        long = "tags",
        value_delimiter = ','
    )]
    pub tags: Vec<String>,
}
