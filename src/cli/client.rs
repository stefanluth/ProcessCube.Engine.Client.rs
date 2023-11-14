use clap::Subcommand;
use serde::Deserialize;

use super::subcommands::{
    application_info::GetApplicationInfo, process_definition::ProcessDefinition,
};

#[derive(Clone, Debug, Deserialize, Subcommand)]
#[serde(rename_all = "kebab-case")]
pub enum Client {
    /// Get information about the application.
    ApplicationInfo { get: GetApplicationInfo },
    /// Trigger events.
    Event,
    /// Get all flow node instances.
    FlowNodeInstance,
    /// Handle Process Definitions.
    ProcessDefinition {
        #[clap(subcommand)]
        cmd: ProcessDefinition,
    },
    /// Handle Process Models.
    ProcessModel,
}
