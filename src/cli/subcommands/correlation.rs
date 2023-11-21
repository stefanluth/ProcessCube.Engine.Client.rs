use clap::Subcommand;
use serde::Deserialize;

use crate::clients::client_factory::ClientFactory;

#[derive(Clone, Debug, Deserialize, Subcommand)]
#[serde(rename_all = "kebab-case")]
pub enum CorrelationCommands {
    /// Gets all Correlations.
    GetAll,
    /// Gets a single Correlation by ID.
    GetById {
        /// The ID of the Correlation to retrieve.
        id: String,
    },
}

pub async fn register_commands(client_factory: ClientFactory, cmd: CorrelationCommands) {
    let client = client_factory.create_correlation_client();
    match cmd {
        CorrelationCommands::GetAll => match client.get_correlations().await {
            Ok(correlations) => println!("{:#?}", correlations),
            Err(e) => eprintln!("Error getting correlation: {:#?}", e),
        },
        CorrelationCommands::GetById { id } => match client.get_correlation_by_id(&id).await {
            Ok(correlation) => println!("{:#?}", correlation),
            Err(e) => eprintln!("Error getting correlation: {:#?}", e),
        },
    }
}
