use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::flow_node_instance::{FlowNodeInstanceList, FlowNodeInstancesQuery};

const FLOW_NODE_INSTANCES_ENDPOINT: &str = "/flow_node_instances";

/// A client for communicating with the 5Minds Engine's FlowNodeInstance API.
#[derive(Clone)]
pub struct FlowNodeInstanceClient {
    api_client: ApiClient,
    pub flow_node_instance_url: String,
}

impl FlowNodeInstanceClient {
    /// Creates a new instance of the FlowNodeInstanceClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{api::api_client::ApiClient, flow_node_instance::flow_node_instance_client::FlowNodeInstanceClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running 5Minds Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     let flow_node_instance_client = FlowNodeInstanceClient::new(api_client);
    ///     // Get all ProcessDefinitions
    ///     let flow_node_instances = flow_node_instance_client
    ///         .get_flow_node_instances(None, None, None)
    ///         .await?;
    ///     println!("ProcessDefinitions: {:#?}", flow_node_instances);
    ///     Ok(())
    /// }
    pub fn new(api_client: ApiClient) -> FlowNodeInstanceClient {
        let flow_node_instance_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            FLOW_NODE_INSTANCES_ENDPOINT
        );
        FlowNodeInstanceClient {
            api_client,
            flow_node_instance_url,
        }
    }

    /// Returns all FlowNodeInstances.
    pub async fn get_flow_node_instances(
        &self,
        offset: Option<u32>,
        limit: Option<u32>,
        query: Option<FlowNodeInstancesQuery>,
    ) -> Result<FlowNodeInstanceList, EngineError> {
        let mut query_params = match query {
            Some(query) => query.to_query_params(),
            None => Vec::new(),
        };

        if let Some(offset) = offset {
            query_params.push(format!("offset={}", offset));
        }
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        let url = match query_params.is_empty() {
            true => self.flow_node_instance_url.clone(),
            false => format!("{}?{}", self.flow_node_instance_url, query_params.join("&")),
        };

        self.api_client.get::<FlowNodeInstanceList>(&url).await
    }
}
