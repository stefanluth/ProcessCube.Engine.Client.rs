use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessModel {
    #[serde(rename = "processModelId")]
    pub id: String,
    #[serde(rename = "processModelName")]
    pub name: Option<String>,
    pub process_definition_id: String,
    pub version: Option<String>,
    pub custom_properties: serde_json::Value,
    pub is_executable: bool,
    pub lane_set: LaneSet,
    pub start_events: Vec<StartEvent>,
    pub end_events: Vec<EndEvent>,
    pub flow_nodes: Vec<FlowNode>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessModelList {
    pub process_models: Vec<ProcessModel>,
    pub total_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStartRequest {
    pub start_event_id: String,
    pub correlation_id: String,
    pub initial_token: serde_json::Value,
    pub return_on: String,
    pub end_event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStartResponse {
    pub process_instance_id: String,
    pub correlation_id: String,
    pub end_event_id: String,
    pub token_payload: serde_json::Value,
}

impl Default for ProcessStartResponse {
    fn default() -> Self {
        ProcessStartResponse {
            process_instance_id: String::new(),
            correlation_id: String::new(),
            end_event_id: String::new(),
            token_payload: serde_json::Value::Null,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LaneSet {
    lanes: Vec<Lane>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lane {
    id: String,
    extension_elements: Option<ExtensionElements>,
    flow_node_references: Option<Vec<String>>,
    name: Option<String>,
    child_lane_set: Option<Vec<String>>,
    documentation: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionElements {
    camunda_execution_listener: Option<CamundaExecutionListener>,
    camunda_extension_properties: Vec<CamundaExtensionProperties>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CamundaExecutionListener {
    class: String,
    event: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CamundaExtensionProperties {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartEvent {
    id: String,
    name: String,
    process_model_id: String,
    process_model_name: String,
    custom_properties: serde_json::Value,
    flow_node_type: String,
    timer_type: Option<String>,
    timer_value: Option<String>,
    message_name: Option<String>,
    message_id: Option<String>,
    signal_name: Option<String>,
    signal_id: Option<String>,
    documentation: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EndEvent {
    id: String,
    name: String,
    process_model_id: String,
    process_model_name: String,
    custom_properties: serde_json::Value,
    flow_node_type: String,
    error_name: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
    message_name: Option<String>,
    message_id: Option<String>,
    signal_name: Option<String>,
    signal_id: Option<String>,
    documentation: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNode {
    id: String,
    name: String,
    custom_properties: serde_json::Value,
    flow_node_type: String,
    process_model_id: String,
    process_model_name: String,
    documentation: Option<Vec<String>>,
}
