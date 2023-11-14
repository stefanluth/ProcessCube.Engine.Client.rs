use clap::Subcommand;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Subcommand)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessDefinition {
    /// Gets all Process Definitions.
    GetAll,
    /// Gets a single Process Definition.
    GetById {
        /// The ID of the Process Definition to retrieve.
        id: String,
    },
    /// Creates or updates a ProcessDefinition.
    Post {
        /// The XML of the Process Definition to create.
        xml: String,
        /// Whether to overwrite an existing Process Definition with the same ID.
        overwrite_existing: Option<bool>,
    },
    /// Deletes a Process Definition.
    Delete {
        /// The ID of the Process Definition to delete.
        id: String,
        /// Whether to delete all related data.
        delete_all_related_data: Option<bool>,
    },
}
