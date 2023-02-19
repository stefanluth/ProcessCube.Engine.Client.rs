use serde::{Deserialize, Serialize};

use crate::clients::process_model::process_model::ProcessModel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessDefinition {
    pub id: String,
    pub xml: String,
    pub hash: String,
    pub process_models: Vec<ProcessModel>,
    pub deployed_at: String,
    pub deployed_by_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessDefinitionList {
    pub process_definitions: Vec<ProcessDefinition>,
    pub total_count: u32,
}
