use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::application_info::ApplicationInfo;

const APPLICATION_INFO_ENDPOINT: &str = "/info";
const AUTHORITY_INFO_ENDPOINT: &str = "/authority";

/// A client for retrieving information about the ProcessCube® Engine and used 5Minds Authority.
#[derive(Clone)]
pub struct ApplicationInfoClient {
    api_client: ApiClient,
    pub application_info_url: String,
    pub authority_info_url: String,
}

impl ApplicationInfoClient {
    /// Creates a new instance of the ApplicationInfoClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the ProcessCube® Engine.
    ///
    /// # Example
    /// ```
    /// use processcube_engine_client::clients::{api::api_client::ApiClient, application_info::application_info_client::ApplicationInfoClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running ProcessCube® Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     let application_info_client = ApplicationInfoClient::new(api_client);
    ///     // Get information about the ProcessCube® Engine
    ///     let application_info = application_info_client.get_application_info().await?;
    ///     println!("Engine version: {}", application_info.version);
    ///     // Get information about the 5Minds Authority
    ///     let authority_info = application_info_client.get_authority_info().await?;
    ///     println!("Authority: {}", authority_info);
    ///     Ok(())
    /// }
    /// ```
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

    /// Returns information about the ProcessCube® Engine.
    pub async fn get_application_info(&self) -> Result<ApplicationInfo, EngineError> {
        self.api_client
            .get::<ApplicationInfo>(&self.application_info_url)
            .await
    }

    /// Returns information about the 5Minds Authority used by the ProcessCube® Engine.
    pub async fn get_authority_info(&self) -> Result<String, EngineError> {
        self.api_client
            .get::<String>(&self.authority_info_url)
            .await
    }
}
