use crate::clients::{
    api::api_client::ApiClient, error::EngineError,
    process_definition::process_definition::ProcessDefinition,
};

use super::process_model::{
    ProcessModel, ProcessModelList, ProcessStartRequest, ProcessStartResponse,
};

const PROCESS_MODELS_ENDPOINT: &str = "/process_models";

/// A client for communicating with the 5Minds Engine's ProcessModel API.
#[derive(Clone)]
pub struct ProcessModelClient {
    api_client: ApiClient,
    pub process_models_url: String,
}

impl ProcessModelClient {
    /// Creates a new instance of the ProcessModelClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{api::api_client::ApiClient, process_model::process_model_client::ProcessModelClient, error::EngineError};
    /// const ENGINE_URL: &str = "http://localhost:10560";
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, "dummy_auth_token");
    ///     let process_model_client = ProcessModelClient::new(api_client);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(api_client: ApiClient) -> ProcessModelClient {
        let process_models_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            PROCESS_MODELS_ENDPOINT
        );
        ProcessModelClient {
            api_client,
            process_models_url,
        }
    }

    /// Returns all ProcessModels deployed to the 5Minds Engine.
    pub async fn get_process_models(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ProcessModelList, EngineError> {
        let mut url = self.process_models_url.clone();
        if let Some(offset) = offset {
            url = format!("{}?offset={}", url, offset);
        }
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }

        self.api_client.get::<ProcessModelList>(&url).await
    }

    /// Returns the ProcessModel with the given ID.
    pub async fn get_process_model_by_id(
        &self,
        process_model_id: &str,
    ) -> Result<ProcessModel, EngineError> {
        let url = format!("{}/{}", self.process_models_url, process_model_id);

        self.api_client.get::<ProcessModel>(&url).await
    }

    /// Deletes the ProcessModel with the given ID.
    pub async fn delete_process_model_by_id(
        &self,
        process_model_id: &str,
    ) -> Result<(), EngineError> {
        let url = format!("{}/{}", self.process_models_url, process_model_id);

        self.api_client.delete::<()>(&url).await
    }

    /// Returns the ProcessDefinition of the ProcessModel with the given ID.
    pub async fn get_process_definition_by_process_model_id(
        &self,
        process_model_id: &str,
    ) -> Result<ProcessDefinition, EngineError> {
        let url = format!(
            "{}/{}/process_definition",
            self.process_models_url, process_model_id
        );

        self.api_client.get::<ProcessDefinition>(&url).await
    }

    /// Starts a new ProcessInstance of the ProcessModel with the given ID.
    pub async fn start_process_instance_by_process_model_id(
        &self,
        process_model_id: &str,
        request: ProcessStartRequest,
    ) -> Result<ProcessStartResponse, EngineError> {
        let url = format!("{}/{}/start", self.process_models_url, process_model_id);
        let request_json = serde_json::to_value(request).expect("Failed to serialize request");

        self.api_client
            .post::<ProcessStartResponse>(&url, Some(&request_json))
            .await
    }

    /// Enables the ProcessModel with the given ID.
    pub async fn enable_process_model_by_id(
        &self,
        process_model_id: &str,
    ) -> Result<(), EngineError> {
        let url = format!("{}/{}/enable", self.process_models_url, process_model_id);

        self.api_client.post::<()>(&url, None).await
    }

    /// Disables the ProcessModel with the given ID.
    pub async fn disable_process_model_by_id(
        &self,
        process_model_id: &str,
    ) -> Result<(), EngineError> {
        let url = format!("{}/{}/disable", self.process_models_url, process_model_id);

        self.api_client.post::<()>(&url, None).await
    }
}
