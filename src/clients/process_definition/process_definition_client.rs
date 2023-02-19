use crate::clients::api::api_client::ApiClient;

use super::process_definition::{ProcessDefinition, ProcessDefinitionList};

const PROCESS_DEFINITIONS_ENDPOINT: &str = "/process_definitions";

#[derive(Clone)]
pub struct ProcessDefinitionClient {
    api_client: ApiClient,
    pub process_definitions_url: String,
}

impl ProcessDefinitionClient {
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

    pub async fn get_process_definitions(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ProcessDefinitionList, reqwest::Error> {
        let mut url = self.process_definitions_url.clone();
        if let Some(offset) = offset {
            url = format!("{}?offset={}", url, offset);
        }
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        let response = self
            .api_client
            .http_client
            .get(&url)
            .header("Authorization", self.api_client.get_auth_token())
            .send()
            .await?
            .json::<ProcessDefinitionList>()
            .await?;
        Ok(response)
    }

    pub async fn get_process_definition_by_id(
        &self,
        process_definition_id: &str,
    ) -> Result<ProcessDefinition, reqwest::Error> {
        let url = format!("{}/{}", self.process_definitions_url, process_definition_id);
        let response = self
            .api_client
            .http_client
            .get(&url)
            .header("Authorization", self.api_client.get_auth_token())
            .send()
            .await?
            .json::<ProcessDefinition>()
            .await?;
        Ok(response)
    }

    pub async fn upload_process_definition(
        &self,
        xml: &str,
        override_existing: Option<bool>,
    ) -> Result<ProcessDefinition, reqwest::Error> {
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
            .await?
            .json::<ProcessDefinition>()
            .await?;
        Ok(response)
    }

    pub async fn delete_process_definition_by_id(
        &self,
        process_definition_id: &str,
        delete_all_related_data: Option<bool>,
    ) -> Result<(), reqwest::Error> {
        let url = format!(
            "{}/{}?deleteAllRelatedData={}",
            self.process_definitions_url,
            process_definition_id,
            delete_all_related_data.unwrap_or(false)
        );
        self.api_client
            .http_client
            .delete(&url)
            .header("Authorization", self.api_client.get_auth_token())
            .send()
            .await?;
        Ok(())
    }
}
