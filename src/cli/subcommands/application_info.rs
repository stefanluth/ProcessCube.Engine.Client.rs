use clap::ValueEnum;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum GetApplicationInfo {
    /// Gets some basic info about the host application.
    Info,
    /// Gets the address of the authority that the host application uses for claim checks.
    Authority,
}
