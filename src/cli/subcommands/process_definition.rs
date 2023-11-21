use clap::Subcommand;
use serde::Deserialize;

use crate::clients::{
    client_factory::ClientFactory,
    process_definition::process_definition::PersistProcessDefinitionPayload,
};

#[derive(Clone, Debug, Deserialize, Subcommand)]
#[serde(rename_all = "kebab-case")]
pub enum ProcessDefinitionCommands {
    /// Gets all Process Definitions.
    GetAll,
    /// Gets a single Process Definition by ID.
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

pub async fn register_commands(client_factory: ClientFactory, cmd: ProcessDefinitionCommands) {
    let client = client_factory.create_process_definition_client();

    match cmd {
        ProcessDefinitionCommands::GetAll => {
            match client.get_process_definitions(None, None).await {
                Ok(process_definitions) => println!("{:#?}", process_definitions),
                Err(e) => eprintln!("Error getting process definitions: {:#?}", e),
            }
        }
        ProcessDefinitionCommands::GetById { id } => {
            match client.get_process_definition_by_id(&id).await {
                Ok(process_definition) => println!("{:#?}", process_definition),
                Err(e) => eprintln!("Error getting process definition: {:#?}", e),
            }
        }
        ProcessDefinitionCommands::Post {
            xml,
            overwrite_existing,
        } => {
            let request = PersistProcessDefinitionPayload {
                xml,
                overwrite_existing: match overwrite_existing {
                    Some(overwrite_existing) => overwrite_existing,
                    None => false,
                },
            };
            match client.upload_process_definition(request).await {
                Ok(_) => println!("Process definition uploaded"),
                Err(e) => eprintln!("Error uploading process definition: {:#?}", e),
            }
        }
        ProcessDefinitionCommands::Delete {
            id,
            delete_all_related_data,
        } => match client
            .delete_process_definition_by_id(&id, delete_all_related_data)
            .await
        {
            Ok(_) => println!("Process definition deleted"),
            Err(e) => eprintln!("Error deleting process definition: {:#?}", e),
        },
    }
}
