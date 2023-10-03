use serde::{Deserialize, Serialize};

/// Describes a list of FlowNodeInstances.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeInstanceList {
    /// The FlowNodeInstances.
    flow_node_instances: Vec<FlowNodeInstance>,
    /// The total number of FlowNodeInstances.
    total_count: u32,
}

/// Describes a FlowNodeInstance.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeInstance {
    correlation_id: String,
    flow_node_id: String,
    flow_node_instance_id: String,
    flow_node_type: BpmnType,
    owner_id: String,
    process_definition_id: String,
    process_instance_id: String,
    process_model_id: String,
    start_token: std::collections::HashMap<String, serde_json::Value>,
    state: FlowNodeInstanceState,
    tokens: Vec<ProcessToken>,

    end_token: Option<std::collections::HashMap<String, serde_json::Value>>,
    error: Option<serde_json::Value>,
    event_type: Option<EventType>,
    finished_at: Option<String>,
    flow_node_lane: Option<String>,
    flow_node_name: Option<String>,
    parent_process_instance_id: Option<String>,
    previous_flow_node_instance_id: Option<String>,
    started_at: Option<String>,
    triggered_by_flow_node_instance: Option<Box<FlowNodeInstance>>,
}

/// The state of a FlowNodeInstance.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FlowNodeInstanceState {
    Canceled,
    Error,
    Finished,
    Running,
    Suspended,
    Terminated,
}

impl FlowNodeInstanceState {
    pub fn as_str(&self) -> &'static str {
        match *self {
            FlowNodeInstanceState::Canceled => "canceled",
            FlowNodeInstanceState::Error => "error",
            FlowNodeInstanceState::Finished => "finished",
            FlowNodeInstanceState::Running => "running",
            FlowNodeInstanceState::Suspended => "suspended",
            FlowNodeInstanceState::Terminated => "terminated",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessToken {
    created_at: String,
    flow_node_instance_id: String,
    payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeInstancesQuery {
    pub correlation_id: Option<String>,
    pub created_at: Option<String>,
    pub event_type: Option<String>,
    pub flow_node_id: Option<String>,
    pub flow_node_instance_id: Option<String>,
    pub flow_node_lane: Option<String>,
    pub flow_node_name: Option<String>,
    pub flow_node_type: Option<String>,
    pub owner_id: Option<String>,
    pub parent_process_instance_id: Option<String>,
    pub previous_flow_node_instance_id: Option<String>,
    pub process_definition_id: Option<String>,
    pub process_instance_id: Option<String>,
    pub process_model_id: Option<String>,
    pub state: Option<String>,
    pub updated_at: Option<String>,
}

impl FlowNodeInstancesQuery {
    pub fn to_query_params(&self) -> Vec<String> {
        let mut parts = Vec::new();

        fn append_param(parts: &mut Vec<String>, key: &str, value: &Option<String>) {
            if let Some(ref val) = value {
                parts.push(format!("{}={}", key, val));
            }
        }

        append_param(
            &mut parts,
            "flowNodeInstanceId",
            &self.flow_node_instance_id,
        );
        append_param(&mut parts, "flowNodeId", &self.flow_node_id);
        append_param(&mut parts, "flowNodeName", &self.flow_node_name);
        append_param(&mut parts, "flowNodeLane", &self.flow_node_lane);
        append_param(&mut parts, "flowNodeType", &self.flow_node_type);
        append_param(&mut parts, "eventType", &self.event_type);
        append_param(&mut parts, "correlationId", &self.correlation_id);
        append_param(
            &mut parts,
            "processDefinitionId",
            &self.process_definition_id,
        );
        append_param(&mut parts, "processModelId", &self.process_model_id);
        append_param(&mut parts, "processInstanceId", &self.process_instance_id);
        append_param(&mut parts, "ownerId", &self.owner_id);
        append_param(&mut parts, "state", &self.state);
        append_param(
            &mut parts,
            "previousFlowNodeInstanceId",
            &self.previous_flow_node_instance_id,
        );
        append_param(
            &mut parts,
            "parentProcessInstanceId",
            &self.parent_process_instance_id,
        );

        append_param(&mut parts, "createdAt", &self.created_at);
        append_param(&mut parts, "updatedAt", &self.updated_at);

        parts
    }
}

/// The type of a BPMN element.
#[derive(Serialize, Deserialize, Debug)]
pub enum BpmnType {
    BoundaryEvent,
    BusinessRuleTask,
    CallActivity,
    ComplexGateway,
    EmptyActivity,
    EndEvent,
    EventBasedGateway,
    ExclusiveGateway,
    InclusiveGateway,
    IntermediateCatchEvent,
    IntermediateThrowEvent,
    ManualTask,
    ParallelGateway,
    ReceiveTask,
    ScriptTask,
    SendTask,
    ServiceTask,
    StartEvent,
    SubProcess,
    UserTask,
}

impl BpmnType {
    /// Contains a list of all known BPMN types and maps them to their corresponding XML tag.
    pub fn as_str(&self) -> &'static str {
        match *self {
            BpmnType::BoundaryEvent => "bpmn:BoundaryEvent",
            BpmnType::BusinessRuleTask => "bpmn:BusinessRuleTask",
            BpmnType::CallActivity => "bpmn:CallActivity",
            BpmnType::ComplexGateway => "bpmn:ComplexGateway",
            BpmnType::EmptyActivity => "bpmn:EmptyActivity",
            BpmnType::EndEvent => "bpmn:EndEvent",
            BpmnType::EventBasedGateway => "bpmn:EventBasedGateway",
            BpmnType::ExclusiveGateway => "bpmn:ExclusiveGateway",
            BpmnType::InclusiveGateway => "bpmn:InclusiveGateway",
            BpmnType::IntermediateCatchEvent => "bpmn:IntermediateCatchEvent",
            BpmnType::IntermediateThrowEvent => "bpmn:IntermediateThrowEvent",
            BpmnType::ManualTask => "bpmn:ManualTask",
            BpmnType::ParallelGateway => "bpmn:ParallelGateway",
            BpmnType::ReceiveTask => "bpmn:ReceiveTask",
            BpmnType::ScriptTask => "bpmn:ScriptTask",
            BpmnType::SendTask => "bpmn:SendTask",
            BpmnType::ServiceTask => "bpmn:ServiceTask",
            BpmnType::StartEvent => "bpmn:StartEvent",
            BpmnType::SubProcess => "bpmn:SubProcess",
            BpmnType::UserTask => "bpmn:UserTask",
        }
    }
}

/// The type of an event.
#[derive(Serialize, Deserialize, Debug)]
pub enum EventType {
    ErrorEvent,
    LinkEvent,
    MessageEvent,
    SignalEvent,
    TerminateEvent,
    TimerEvent,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            EventType::ErrorEvent => "errorEvent",
            EventType::LinkEvent => "linkEvent",
            EventType::MessageEvent => "messageEvent",
            EventType::SignalEvent => "signalEvent",
            EventType::TerminateEvent => "terminateEvent",
            EventType::TimerEvent => "timerEvent",
        }
    }
}
