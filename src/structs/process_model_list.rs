use serde::{Deserialize, Serialize};
use super::process_model::ProcessModel;


#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessModelList {
    #[serde(rename = "processModels")]
    pub process_models: Vec<ProcessModel>,
    #[serde(rename = "totalCount")]
    pub total_count: u32,
}
