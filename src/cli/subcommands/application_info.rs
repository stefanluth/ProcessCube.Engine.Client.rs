use clap::ValueEnum;
use serde::Deserialize;

use crate::clients::client_factory::ClientFactory;

#[derive(Clone, Debug, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ApplicationInfoCommands {
    /// Gets some basic info about the host application.
    GetInfo,
    /// Gets the address of the authority that the host application uses for claim checks.
    GetAuthority,
}

pub async fn register_commands(client_factory: ClientFactory, get: ApplicationInfoCommands) {
    let client = client_factory.create_application_info_client();
    match get {
        ApplicationInfoCommands::GetInfo => match client.get_application_info().await {
            Ok(info) => println!("{:#?}", info),
            Err(e) => eprintln!("Error getting application info: {:#?}", e),
        },
        ApplicationInfoCommands::GetAuthority => match client.get_authority_info().await {
            Ok(authority) => println!("{:#?}", authority),
            Err(e) => eprintln!("Error getting authority: {:#?}", e),
        },
    }
}
