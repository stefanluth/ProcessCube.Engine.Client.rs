use super::{
    api::api_client::ApiClient, application_info::application_info_client::ApplicationInfoClient,
    event::event_client::EventClient,
    process_definition::process_definition_client::ProcessDefinitionClient,
    process_model::process_model_client::ProcessModelClient,
};

/// A factory for creating clients for the 5Minds Engine.
pub struct ClientFactory {
    /// The client used to communicate with the 5Minds Engine.
    pub api_client: ApiClient,
}

impl ClientFactory {
    /// Creates a new instance of the ClientFactory.
    ///
    /// # Arguments
    /// * `engine_url` - The URL of the 5Minds Engine.
    /// * `auth_token` - The authentication token to use when communicating with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{client_factory::ClientFactory, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running 5Minds Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    ///     // Create a new ApplicationInfoClient
    ///     let application_info_client = client_factory.create_application_info_client();
    ///     // Create a new ProcessDefinitionClient
    ///     let process_definition_client = client_factory.create_process_definition_client();
    ///     // Create a new ProcessModelClient
    ///     let process_model_client = client_factory.create_process_model_client();
    ///     // Create a new EventClient
    ///     let event_client = client_factory.create_event_client();
    ///     Ok(())
    /// }
    /// ```
    pub fn new(engine_url: &str, auth_token: &str) -> ClientFactory {
        let api_client = ApiClient::new(engine_url, auth_token);
        ClientFactory { api_client }
    }

    /// Creates a new instance of the ApplicationInfoClient.
    pub fn create_application_info_client(&self) -> ApplicationInfoClient {
        ApplicationInfoClient::new(self.api_client.clone())
    }

    /// Creates a new instance of the ProcessDefinitionClient.
    pub fn create_process_definition_client(&self) -> ProcessDefinitionClient {
        ProcessDefinitionClient::new(self.api_client.clone())
    }

    /// Creates a new instance of the ProcessModelClient.
    pub fn create_process_model_client(&self) -> ProcessModelClient {
        ProcessModelClient::new(self.api_client.clone())
    }

    /// Creates a new instance of the EventClient.
    pub fn create_event_client(&self) -> EventClient {
        EventClient::new(self.api_client.clone())
    }
}
