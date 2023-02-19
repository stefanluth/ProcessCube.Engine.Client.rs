use reqwest::Client;

const ENGINE_API_ENDPOINT: &str = "/atlas_engine/api/v1";

#[derive(Clone)]
pub struct ApiClient {
    pub http_client: Client,
    engine_url: String,
    auth_token: String,
}

impl ApiClient {
    pub fn new(engine_url: &str, auth_token: &str) -> ApiClient {
        let http_client = Client::new();
        ApiClient {
            http_client,
            engine_url: engine_url.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    pub fn get_engine_url(&self) -> &str {
        &self.engine_url
    }

    pub fn get_engine_api_endpoint(&self) -> &str {
        ENGINE_API_ENDPOINT
    }

    pub fn get_auth_token(&self) -> &str {
        &self.auth_token
    }
}
