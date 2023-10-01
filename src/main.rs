use engine_client::clients::{client_factory::ClientFactory, error::EngineError};

const ENGINE_URL: &str = "http://localhost:10560";
const AUTH_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";

#[tokio::main]
async fn main() -> Result<(), EngineError> {
    println!("This is a sample application to demonstrate the usage of the engine client library.");
    println!(
        "The application will connect to the engine at {} using the dummy auth token {}.",
        ENGINE_URL, AUTH_TOKEN
    );
    println!("Make sure that the engine is running at the specified URL before continuing.");

    let client_factory = ClientFactory::new(ENGINE_URL, AUTH_TOKEN);

    println!("\n------------------- Application Info Client -------------------");
    let application_info_client = client_factory.create_application_info_client();
    let response = application_info_client.get_application_info().await?;
    println!("------------------- Application Info -------------------");
    println!("{:#?}", response);

    let response = application_info_client.get_authority_info().await?;
    println!("------------------- Authority Info -------------------");
    println!("{:#?}", response);

    println!("\n------------------- Process Definition Client -------------------");
    let process_definition_client = client_factory.create_process_definition_client();
    let response = process_definition_client
        .get_process_definitions(None, None)
        .await?;
    println!("------------------- Process Definitions -------------------");
    println!("{:#?}", response);

    if let Err(response) = process_definition_client
        .get_process_definition_by_id("this_should_fail")
        .await
    {
        println!("------------------- Process Definition Error Expected -------------------");
        println!("{:#?}", response);
    }

    println!("\n------------------- Process Model Client -------------------");
    let process_model_client = client_factory.create_process_model_client();
    let response = process_model_client.get_process_models(None, None).await?;
    println!("------------------- Process Models -------------------");
    println!("{:#?}", response);

    Ok(())
}
