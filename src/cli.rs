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

/// リンクはオートメーションです
///
/// It's dangerous to go alone! Take this...
#[derive(Parser)]
pub struct Cli {
    pub linkfile: path::PathBuf,

    #[arg(
        value_enum,
        short = 'm',
        long = "mode",
        default_value_t = Mode::Dry
    )]
    pub mode: Mode,
}
