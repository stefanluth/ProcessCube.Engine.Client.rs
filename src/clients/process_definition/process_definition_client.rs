use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::process_definition::{
    PersistProcessDefinitionPayload, ProcessDefinition, ProcessDefinitionList,
};

const PROCESS_DEFINITIONS_ENDPOINT: &str = "/process_definitions";

/// A client for communicating with the ProcessCube® Engine's ProcessDefinition API.
#[derive(Clone)]
pub struct ProcessDefinitionClient {
    api_client: ApiClient,
    pub process_definitions_url: String,
}

impl ProcessDefinitionClient {
    /// Creates a new instance of the ProcessDefinitionClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the ProcessCube® Engine.
    ///
    /// # Example
    /// ```
    /// use processcube_engine_client::clients::{api::api_client::ApiClient, process_definition::process_definition_client::ProcessDefinitionClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running ProcessCube® Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     let process_definition_client = ProcessDefinitionClient::new(api_client);
    ///     // Get all ProcessDefinitions
    ///     let process_definitions = process_definition_client
    ///         .get_process_definitions(None, None)
    ///         .await?;
    ///     println!("ProcessDefinitions: {:#?}", process_definitions);
    ///     Ok(())
    /// }
    pub fn new(api_client: ApiClient) -> ProcessDefinitionClient {
        let process_definitions_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            PROCESS_DEFINITIONS_ENDPOINT
        );
        ProcessDefinitionClient {
            api_client,
            process_definitions_url,
        }
    }

    /// Returns all ProcessDefinitions deployed to the ProcessCube® Engine.
    pub async fn get_process_definitions(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ProcessDefinitionList, EngineError> {
        let mut query_params = Vec::new();
        if let Some(offset) = offset {
            query_params.push(format!("offset={}", offset));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        let url = match query_params.is_empty() {
            true => self.process_definitions_url.clone(),
            false => format!(
                "{}?{}",
                self.process_definitions_url,
                query_params.join("&")
            ),
        };

        self.api_client.get::<ProcessDefinitionList>(&url).await
    }

    /// Returns the ProcessDefinition with the given ID.
    pub async fn get_process_definition_by_id(
        &self,
        process_definition_id: &str,
    ) -> Result<ProcessDefinition, EngineError> {
        let url = format!("{}/{}", self.process_definitions_url, process_definition_id);

        self.api_client.get::<ProcessDefinition>(&url).await
    }

    /// Uploads a new ProcessDefinition to the ProcessCube® Engine.
    pub async fn upload_process_definition(
        &self,
        request: PersistProcessDefinitionPayload,
    ) -> Result<(), EngineError> {
        let request_json = serde_json::to_value(request).expect("Failed to serialize request");

        self.api_client
            .post::<()>(&self.process_definitions_url, Some(&request_json))
            .await
    }

    /// Deletes the ProcessDefinition with the given ID.
    pub async fn delete_process_definition_by_id(
        &self,
        process_definition_id: &str,
        delete_all_related_data: Option<bool>,
    ) -> Result<(), EngineError> {
        let url = format!(
            "{}/{}?deleteAllRelatedData={}",
            self.process_definitions_url,
            process_definition_id,
            delete_all_related_data.unwrap_or(false)
        );

        self.api_client.delete::<()>(&url).await
    }
}
