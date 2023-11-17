use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::clients::client_factory::ClientFactory;

use super::subcommands;

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    client: Client,

    #[clap(short, long, default_value = DUMMY_TOKEN)]
    token: String,

    #[clap(short, long, default_value = ENGINE_URL)]
    engine_url: String,
}

#[derive(Clone, Debug, Deserialize, Subcommand)]
#[serde(rename_all = "kebab-case")]
pub enum Client {
    /// Get information about the application.
    ApplicationInfo {
        cmd: subcommands::application_info::ApplicationInfoCommands,
    },
    /// Trigger events.
    Event,
    /// Get all flow node instances.
    FlowNodeInstance,
    /// Handle Process Definitions.
    ProcessDefinition {
        #[clap(subcommand)]
        cmd: subcommands::process_definition::ProcessDefinitionCommands,
    },
    /// Handle Process Models.
    ProcessModel,
}

pub async fn register_commands(cli: Cli) {
    let client_factory = ClientFactory::new(&cli.engine_url, &cli.token);

    match cli.client {
        Client::ApplicationInfo { cmd } => {
            subcommands::application_info::register_commands(client_factory, cmd).await
        }
        Client::Event => {
            println!("Event");
        }
        Client::FlowNodeInstance => {
            println!("FlowNodeInstance");
        }
        Client::ProcessDefinition { cmd } => {
            subcommands::process_definition::register_commands(client_factory, cmd).await
        }
        Client::ProcessModel => {
            println!("ProcessModel");
        }
    }
}
