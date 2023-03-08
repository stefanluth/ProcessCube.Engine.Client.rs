use engine_client::clients::{client_factory::ClientFactory, error::EngineError};

const ENGINE_URL: &str = "http://localhost:10560";
const AUTH_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";

#[tokio::main]
async fn main() -> Result<(), EngineError> {
    let client_factory = ClientFactory::new(ENGINE_URL, AUTH_TOKEN);

    // application_info_client
    let application_info_client = client_factory.create_application_info_client();

    let response = application_info_client.get_application_info().await?;
    println!("{:#?}", response);

    let response = application_info_client.get_authority_info().await?;
    println!("{:#?}", response);

    // process_definition_client
    let process_definition_client = client_factory.create_process_definition_client();

    let response = process_definition_client
        .get_process_definitions(None, None)
        .await?;
    println!("{:#?}", response);

    if let Err(response) = process_definition_client
        .get_process_definition_by_id("this_should_fail")
        .await
    {
        println!("{:#?}", response);
    }

    if let Err(response) = process_definition_client
        .delete_process_definition_by_id("this_should_fail", Some(true))
        .await
    {
        println!("{:#?}", response);
    }

    // process_instance_client
    let process_model_client = client_factory.create_process_model_client();

    let response = process_model_client.get_process_models(None, None).await?;
    println!("{:#?}", response);

    println!("End of main function.");

    Ok(())
}
