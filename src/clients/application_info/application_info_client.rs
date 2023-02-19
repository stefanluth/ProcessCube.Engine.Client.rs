use crate::clients::api::api_client::ApiClient;

use super::application_info::ApplicationInfo;

const APPLICATION_INFO_ENDPOINT: &str = "/info";
const AUTHORITY_INFO_ENDPOINT: &str = "/authority";

#[derive(Clone)]
pub struct ApplicationInfoClient {
    api_client: ApiClient,
    pub application_info_url: String,
    pub authority_info_url: String,
}

impl ApplicationInfoClient {
    pub fn new(api_client: ApiClient) -> ApplicationInfoClient {
        let application_info_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            APPLICATION_INFO_ENDPOINT
        );
        let authority_info_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            AUTHORITY_INFO_ENDPOINT
        );
        ApplicationInfoClient {
            api_client,
            application_info_url,
            authority_info_url,
        }
    }

    pub async fn get_application_info(&self) -> Result<ApplicationInfo, reqwest::Error> {
        let response = self
            .api_client
            .http_client
            .get(&self.application_info_url)
            .header("Authorization", self.api_client.get_auth_token())
            .send()
            .await?
            .json::<ApplicationInfo>()
            .await?;
        Ok(response)
    }

    pub async fn get_authority_info(&self) -> Result<String, reqwest::Error> {
        let response = self
            .api_client
            .http_client
            .get(&self.authority_info_url)
            .header("Authorization", self.api_client.get_auth_token())
            .send()
            .await?
            .json::<String>()
            .await?;
        Ok(response)
    }
}
