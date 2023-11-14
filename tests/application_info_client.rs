use processcube_engine_client::clients::client_factory::ClientFactory;

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";

#[tokio::test]
async fn get_application_info() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_application_info_client();
    let result = client.get_application_info().await;

    if let Err(e) = result {
        panic!("Error getting application info: {:#?}", e);
    }

    let application_info = result.unwrap();

    assert_ne!(application_info.id, None);
    assert_eq!(application_info.name, "5Minds Engine");
    assert_eq!(application_info.package_name, "@5minds/processcube_engine");
    assert_eq!(application_info.authority_url, "http://localhost:11560/");
    assert_ne!(application_info.version, "");
    assert!(application_info.allow_anonymous_root_access);
}

#[tokio::test]
async fn get_authority_info() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_application_info_client();
    let result = client.get_authority_info().await;

    if let Err(e) = result {
        panic!("Error getting authority info: {:#?}", e);
    }

    let authority_info = result.unwrap();

    assert_eq!(authority_info, "http://localhost:11560/");
}
