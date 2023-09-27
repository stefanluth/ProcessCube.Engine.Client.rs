use reqwest::Client;

const ENGINE_API_ENDPOINT: &str = "/atlas_engine/api/v1";

/// A client for communicating with the 5Minds Engine.
///
/// It is used as a base for all other clients and uses the reqwest Client for HTTP communication.
#[derive(Clone)]
pub struct ApiClient {
    pub http_client: Client,
    engine_url: String,
    auth_token: String,
}

impl ApiClient {
    /// Creates a new instance of the ApiClient.
    ///
    /// # Arguments
    /// * `engine_url` - The URL of the 5Minds Engine.
    /// * `auth_token` - The authentication token to use when communicating with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{api::api_client::ApiClient, error::EngineError};
    /// const ENGINE_URL: &str = "http://localhost:10560";
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///   let api_client = ApiClient::new(ENGINE_URL, "dummy_auth_token");
    ///  Ok(())
    /// }
    /// ```
    pub fn new(engine_url: &str, auth_token: &str) -> ApiClient {
        let http_client = Client::new();
        ApiClient {
            http_client,
            engine_url: engine_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    /// Returns the URL of the 5Minds Engine.
    pub fn get_engine_url(&self) -> &str {
        &self.engine_url
    }

    /// Returns the endpoint of the 5Minds Engine API.
    pub fn get_engine_api_endpoint(&self) -> &str {
        ENGINE_API_ENDPOINT
    }

    /// Returns the authentication token used by the ApiClient.
    pub fn get_auth_token(&self) -> &str {
        &self.auth_token
    }
}
