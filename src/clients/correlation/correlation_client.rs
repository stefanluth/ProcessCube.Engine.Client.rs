use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::correlation::{Correlation, CorrelationList};

const CORRELATIONS_ENDPOINT: &str = "/correlations";

/// A client for retrieving correlations from the ProcessCube® Engine.
#[derive(Clone)]
pub struct CorrelationClient {
    api_client: ApiClient,
    pub correlation_url: String,
}

impl CorrelationClient {
    /// Creates a new instance of the CorrelationClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the ProcessCube® Engine.
    ///
    /// # Example
    /// ```
    /// use processcube_engine_client::clients::{api::api_client::ApiClient, correlation::correlation_client::CorrelationClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running ProcessCube® Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     let correlation_client = CorrelationClient::new(api_client);
    ///     // Get all correlations from the ProcessCube® Engine
    ///     let correlations = correlation_client.get_correlations().await?;
    ///     println!("Correlations: {:#?}", correlations);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(api_client: ApiClient) -> CorrelationClient {
        let correlation_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            CORRELATIONS_ENDPOINT
        );
        CorrelationClient {
            api_client,
            correlation_url,
        }
    }

    /// Returns all correlations from the ProcessCube® Engine.
    pub async fn get_correlations(&self) -> Result<CorrelationList, EngineError> {
        self.api_client
            .get::<CorrelationList>(&self.correlation_url)
            .await
    }

    /// Returns a correlation with the given id from the ProcessCube® Engine.
    pub async fn get_correlation_by_id(&self, id: &str) -> Result<Correlation, EngineError> {
        let url = format!("{}/{}", self.correlation_url, id);
        self.api_client.get::<Correlation>(&url).await
    }
}
