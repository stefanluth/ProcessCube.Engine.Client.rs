use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessModel {
    #[serde(rename = "processDefinitionId")]
    pub definition_id: String,
    #[serde(rename = "processModelId")]
    pub id: String,
    #[serde(rename = "processModelName")]
    pub name: String,
    #[serde(rename = "isExecutable")]
    pub is_executable: bool,
    // #[serde(rename = "laneSet")]
    // pub lane_set: Option<LaneSet>,
    // #[serde(rename = "startEvents")]
    // pub start_events: Vec<StartEventViewModel>,
    // #[serde(rename = "endEvents")]
    // pub end_events: Vec<EndEventViewModel>,
    // #[serde(rename = "flowNodes")]
    // pub flow_nodes: Vec<FlowNodeViewModel>,
}
