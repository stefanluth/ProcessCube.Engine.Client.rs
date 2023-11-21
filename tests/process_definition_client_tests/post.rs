use processcube_engine_client::clients::{
    client_factory::ClientFactory,
    process_definition::process_definition::PersistProcessDefinitionPayload,
};

// Happy cases

use crate::fixtures::{get_valid_process_definition_xml, DUMMY_TOKEN, ENGINE_URL};
#[tokio::test]
async fn upload_process_definition() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_process_definition_client();

    // Upload a process definition
    let result = client
        .upload_process_definition(PersistProcessDefinitionPayload {
            xml: get_valid_process_definition_xml(),
            overwrite_existing: true,
        })
        .await;
    assert!(result.is_ok(), "Expected Ok result, but got {:?}", result);

    // Assert that at least one process definition exists
    let result = client.get_process_definitions(None, None).await;
    assert!(result.is_ok(), "Expected Ok result, but got {:?}", result);
    assert!(result.unwrap().process_definitions.len() > 0);
}

// Error cases

#[tokio::test]
async fn upload_process_definition_invalid_token() {
    let client_factory = ClientFactory::new(ENGINE_URL, "foo");
    let client = client_factory.create_process_definition_client();

    // Upload a process definition
    let result = client
        .upload_process_definition(PersistProcessDefinitionPayload {
            xml: get_valid_process_definition_xml(),
            overwrite_existing: true,
        })
        .await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.code, 400);
    assert_eq!(err.error_type, "BadRequestError");
    assert_eq!(
        err.message,
        "Must provide a token by which to create an identity!"
    )
}

#[tokio::test]
async fn upload_process_definition_invalid_xml() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_process_definition_client();

    // Upload a process definition
    let result = client
        .upload_process_definition(PersistProcessDefinitionPayload {
            xml: "foo".to_string(),
            overwrite_existing: true,
        })
        .await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.code, 400);
    assert_eq!(err.error_type, "BadRequestError");
    assert!(err
        .message
        .starts_with("Error: Non-whitespace before first tag."));
}
