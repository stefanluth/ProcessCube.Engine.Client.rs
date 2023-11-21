use processcube_engine_client::clients::client_factory::ClientFactory;

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";

#[test]
fn create_application_info_client() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let _client = client_factory.create_application_info_client();
}

#[test]
fn create_correlation_client() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let _client = client_factory.create_correlation_client();
}

#[test]
fn create_event_client() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let _client = client_factory.create_event_client();
}

#[test]
fn create_process_definition_client() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let _client = client_factory.create_process_definition_client();
}

#[test]
fn create_process_model_client() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let _client = client_factory.create_process_model_client();
}
