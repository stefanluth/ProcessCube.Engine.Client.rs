use clap::Parser;

use engine_client::{
    cli::{
        client::Client,
        subcommands::{
            application_info::GetApplicationInfo, process_definition::ProcessDefinition,
        },
    },
    clients::{
        client_factory::ClientFactory,
        process_definition::process_definition::PersistProcessDefinitionPayload,
    },
};

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    client: Client,

    #[clap(short, long, default_value = DUMMY_TOKEN)]
    token: String,

    #[clap(short, long, default_value = ENGINE_URL)]
    engine_url: String,
}

#[tokio::main]
async fn main() -> () {
    let cli = Cli::parse();
    let client_factory = ClientFactory::new(&cli.engine_url, &cli.token);

    match cli.client {
        Client::ApplicationInfo { get } => {
            let client = client_factory.create_application_info_client();
            match get {
                GetApplicationInfo::Info => match client.get_application_info().await {
                    Ok(info) => println!("{:#?}", info),
                    Err(e) => eprintln!("Error getting application info: {:#?}", e),
                },
                GetApplicationInfo::Authority => match client.get_authority_info().await {
                    Ok(authority) => println!("{:#?}", authority),
                    Err(e) => eprintln!("Error getting authority: {:#?}", e),
                },
            }
        }
        Client::Event => {
            println!("Event");
        }
        Client::FlowNodeInstance => {
            println!("FlowNodeInstance");
        }
        Client::ProcessDefinition { cmd } => {
            let client = client_factory.create_process_definition_client();

            match cmd {
                ProcessDefinition::GetAll => {
                    match client.get_process_definitions(None, None).await {
                        Ok(process_definitions) => println!("{:#?}", process_definitions),
                        Err(e) => eprintln!("Error getting process definitions: {:#?}", e),
                    }
                }
                ProcessDefinition::GetById { id } => {
                    match client.get_process_definition_by_id(&id).await {
                        Ok(process_definition) => println!("{:#?}", process_definition),
                        Err(e) => eprintln!("Error getting process definition: {:#?}", e),
                    }
                }
                ProcessDefinition::Post {
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
                ProcessDefinition::Delete {
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
        Client::ProcessModel => {
            println!("ProcessModel");
        }
    }
}
