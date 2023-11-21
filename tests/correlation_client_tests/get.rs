use processcube_engine_client::clients::client_factory::ClientFactory;

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";

// Happy cases

#[tokio::test]
async fn get_correlations() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_correlation_client();
    let result = client.get_correlations().await;
    assert!(result.is_ok());
}

// TODO: Add happy case test for get_correlation_by_id once ProcessInstance is implemented

// Error cases

#[tokio::test]
async fn get_correlation_by_id_not_found() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_correlation_client();
    let result = client.get_correlation_by_id("dummy").await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code, 404);
    assert_eq!(error.error_type, "NotFoundError");
    assert_eq!(error.message, "Correlation with ID `dummy` not found.");
}
