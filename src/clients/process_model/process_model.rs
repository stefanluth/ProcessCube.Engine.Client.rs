use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessModel {
    pub process_definition_id: String,
    pub process_model_id: String,
    pub process_model_name: Option<String>,
    pub is_executable: bool,
    pub lane_set: Option<Lane>,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lane {
    name: String,
    flow_node_references: Vec<String>,
    child_lane_set: Vec<String>,
    id: String,
    documentation: Vec<String>,
    extension_elements: ExtensionElements,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionElements {
    camunda_execution_listener: CamundaExecutionListener,
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
    timer_type: String,
    timer_value: String,
    message_name: String,
    message_id: String,
    signal_name: String,
    signal_id: String,
    name: String,
    flow_node_type: String,
    id: String,
    custom_properties: serde_json::Value,
    documentation: Vec<String>,
    process_model_id: String,
    process_model_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EndEvent {
    error_name: String,
    error_code: String,
    error_message: String,
    message_name: String,
    message_id: String,
    signal_name: String,
    signal_id: String,
    name: String,
    flow_node_type: String,
    id: String,
    custom_properties: serde_json::Value,
    documentation: Vec<String>,
    process_model_id: String,
    process_model_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNode {
    name: String,
    flow_node_type: String,
    id: String,
    custom_properties: serde_json::Value,
    documentation: Vec<String>,
    process_model_id: String,
    process_model_name: String,
}
