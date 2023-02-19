use super::{
    api::api_client::ApiClient, application_info::application_info_client::ApplicationInfoClient,
    process_definition::process_definition_client::ProcessDefinitionClient,
    process_model::process_model_client::ProcessModelClient,
};
pub struct ClientFactory {
    pub api_client: ApiClient,
}

impl ClientFactory {
    pub fn new(engine_url: &str, auth_token: &str) -> ClientFactory {
        let api_client = ApiClient::new(engine_url, auth_token);
        ClientFactory { api_client }
    }

    pub fn create_application_info_client(&self) -> ApplicationInfoClient {
        ApplicationInfoClient::new(self.api_client.clone())
    }

    pub fn create_process_definition_client(&self) -> ProcessDefinitionClient {
        ProcessDefinitionClient::new(self.api_client.clone())
    }

    pub fn create_process_model_client(&self) -> ProcessModelClient {
        ProcessModelClient::new(self.api_client.clone())
    }
}
