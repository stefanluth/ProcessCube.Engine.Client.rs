use reqwest::Client;

use crate::clients::error::EngineError;

const ENGINE_API_ENDPOINT: &str = "/atlas_engine/api/v1";

/// A client for communicating with the ProcessCube® Engine API.
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
    /// * `engine_url` - The URL of the ProcessCube® Engine.
    /// * `auth_token` - The authentication token to use when communicating with the ProcessCube® Engine.
    ///
    /// # Example
    /// ```
    /// use processcube_engine_client::clients::{api::api_client::ApiClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running ProcessCube® Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     // Read the Engine URL
    ///     println!("Engine URL: {}", api_client.get_engine_url());
    ///     // Read the Engine API endpoint
    ///     println!("Engine API endpoint: {}", api_client.get_engine_api_endpoint());
    ///     // Read the authentication token
    ///     println!("Auth token: {}", api_client.get_auth_token());
    ///     Ok(())
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

    /// Returns The URL of the ProcessCube® Engine.
    pub fn get_engine_url(&self) -> &str {
        &self.engine_url
    }

    /// Returns the endpoint of the ProcessCube® Engine API.
    pub fn get_engine_api_endpoint(&self) -> &str {
        ENGINE_API_ENDPOINT
    }

    /// Returns the authentication token used by the ApiClient.
    pub fn get_auth_token(&self) -> &str {
        &self.auth_token
    }

    /// Sends a GET request to the given URL and returns the response as a deserialized object.
    ///
    /// # Arguments
    /// * `url` - The URL to send the GET request to.
    pub async fn get<T>(&self, url: &str) -> Result<T, EngineError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .http_client
            .get(url)
            .header("Authorization", self.get_auth_token())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            _ => Err(response.json::<EngineError>().await?),
        }
    }

    /// Sends a POST request to the given URL and returns the response as a deserialized object.
    ///
    /// # Arguments
    /// * `url` - The URL to send the POST request to.
    /// * `body` - The body of the POST request as a JSON object. If no body is required, this can be set to `None`.
    pub async fn post<T>(
        &self,
        url: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T, EngineError>
    where
        T: serde::de::DeserializeOwned + Default,
    {
        let response = match body {
            Some(body) => {
                self.http_client
                    .post(url)
                    .header("Authorization", self.get_auth_token())
                    .json(body)
                    .send()
                    .await?
            }
            None => {
                self.http_client
                    .post(url)
                    .header("Authorization", self.get_auth_token())
                    .send()
                    .await?
            }
        };

        match response.status().as_u16() {
            200..=299 => match response.content_length().unwrap_or_default() {
                0 => Ok(serde_json::from_str("{}").unwrap_or_default()),
                _ => Ok(response.json::<T>().await?),
            },
            _ => Err(response.json::<EngineError>().await?),
        }
    }

    /// Sends a DELETE request to the given URL and returns the response as a deserialized object.
    ///
    /// # Arguments
    /// * `url` - The URL to send the DELETE request to.
    pub async fn delete<T>(&self, url: &str) -> Result<T, EngineError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self
            .http_client
            .delete(url)
            .header("Authorization", self.get_auth_token())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            _ => Err(response.json::<EngineError>().await?),
        }
    }
}
