use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationList {
    pub correlations: Vec<Correlation>,
    pub total_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Correlation {
    #[serde(rename = "correlationId")]
    pub id: String,
    pub metadata: serde_json::Value,
    // TODO: pub process_instances: Vec<ProcessInstance>,
}
