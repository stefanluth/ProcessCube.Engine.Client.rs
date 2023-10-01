use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::process_definition::{ProcessDefinition, ProcessDefinitionList};

const PROCESS_DEFINITIONS_ENDPOINT: &str = "/process_definitions";

/// A client for communicating with the 5Minds Engine's ProcessDefinition API.
#[derive(Clone)]
pub struct ProcessDefinitionClient {
    api_client: ApiClient,
    pub process_definitions_url: String,
}

impl ProcessDefinitionClient {
    /// Creates a new instance of the ProcessDefinitionClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{api::api_client::ApiClient, process_definition::process_definition_client::ProcessDefinitionClient, error::EngineError};
    /// const ENGINE_URL: &str = "http://localhost:10560";
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, "dummy_auth_token");
    ///     let process_definition_client = ProcessDefinitionClient::new(api_client);
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

    /// Returns all ProcessDefinitions deployed to the 5Minds Engine.
    pub async fn get_process_definitions(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ProcessDefinitionList, EngineError> {
        let mut url = self.process_definitions_url.clone();
        if let Some(offset) = offset {
            url = format!("{}?offset={}", url, offset);
        }
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }

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

    /// Uploads a new ProcessDefinition to the 5Minds Engine.
    pub async fn upload_process_definition(
        &self,
        xml: &str,
        override_existing: Option<bool>,
    ) -> Result<ProcessDefinition, EngineError> {
        let url = format!(
            "{}?overrideExisting={}",
            self.process_definitions_url,
            override_existing.unwrap_or(false)
        );
        let response = self
            .api_client
            .http_client
            .post(&url)
            .header("Authorization", self.api_client.get_auth_token())
            .header("Content-Type", "application/xml")
            .body(xml.to_string())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<ProcessDefinition>().await?),
            _ => Err(response.json::<EngineError>().await?),
        }
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
