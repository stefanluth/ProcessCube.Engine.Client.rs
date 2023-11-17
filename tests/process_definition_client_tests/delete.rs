use processcube_engine_client::clients::{
    client_factory::ClientFactory,
    process_definition::process_definition::PersistProcessDefinitionPayload,
};

use crate::fixtures::{get_valid_process_definition_xml, DUMMY_TOKEN, ENGINE_URL};

// Happy cases

#[tokio::test]
async fn delete_process_definition_by_id() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_process_definition_client();

    // Upload a process definition
    let _ = client
        .upload_process_definition(PersistProcessDefinitionPayload {
            xml: get_valid_process_definition_xml(),
            overwrite_existing: true,
        })
        .await;

    // Get the ID of the uploaded process definition
    let result = client.get_process_definitions(None, None).await;
    let process_definition_list = result.unwrap();
    let process_definition = process_definition_list.process_definitions.first().unwrap();

    // Delete the process definition by ID
    let result = client
        .delete_process_definition_by_id(&process_definition.id, Some(true))
        .await;
    assert!(result.is_ok(), "Expected Ok result, but got {:?}", result);

    // Get the process definition by ID
    let result = client
        .get_process_definition_by_id(&process_definition.id)
        .await;
    assert!(result.is_err());
}

// Error cases

#[tokio::test]
async fn delete_process_definition_by_id_invalid_token() {
    let client_factory = ClientFactory::new(ENGINE_URL, "foo");
    let client = client_factory.create_process_definition_client();

    // Delete a process definition by ID
    let result = client
        .delete_process_definition_by_id("foo", Some(true))
        .await;
    assert!(result.is_err());
}
