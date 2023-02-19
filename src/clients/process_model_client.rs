use super::engine_client::EngineClient;
use crate::structs::process_model_list::ProcessModelList;

const PROCESS_MODEL_ENDPOINT: &str = "/process_models";

pub struct ProcessModelClient {
    engine_client: EngineClient,
    pub process_model_url: String,
}

impl ProcessModelClient {
    pub fn new(engine_client: EngineClient) -> ProcessModelClient {
        let process_model_url = format!(
            "{}{}{}",
            engine_client.get_engine_url(),
            engine_client.get_engine_api_endpoint(),
            PROCESS_MODEL_ENDPOINT
        );
        ProcessModelClient {
            engine_client,
            process_model_url,
        }
    }

    pub async fn get_process_models(&self) -> Result<ProcessModelList, reqwest::Error> {
        let response = self
            .engine_client
            .http_client
            .get(&self.process_model_url)
            .header("Authorization", self.engine_client.get_auth_token())
            .send()
            .await?
            .json::<ProcessModelList>()
            .await?;
        Ok(response)
    }
}
