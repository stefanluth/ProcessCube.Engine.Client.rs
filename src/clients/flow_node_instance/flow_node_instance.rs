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
    /// The instance ID of this FlowNodeInstance.
    flow_node_instance_id: String,
    /// The unique ID of the FlowNode, as it is modelled in the diagram.
    flow_node_id: String,
    /// The name of the FlowNode, as it is modelled in the diagram.
    flow_node_name: Option<String>,
    /// The lane that this FlowNodeInstance belongs to.
    flow_node_lane: Option<String>,
    /// Describes the BPMN type of the FlowNode that this instance is executing.
    flow_node_type: BpmnType,
    /// If the FlowNodeInstance is an Event, this will contain the type of Event (Message, Signal, Timer, etc).
    event_type: Option<EventType>,
    /// Contains the InstanceId of the FlowNodeInstance that was executed before this one.
    previous_flow_node_instance_id: Option<String>,
    parent_process_instance_id: Option<String>,
    state: FlowNodeInstanceState,
    process_definition_id: String,
    process_model_id: String,
    process_instance_id: String,
    correlation_id: String,
    /// The ProcessToken of the FlowNodeInstance.
    tokens: Vec<ProcessToken>,
    /// The token with which the Flow Node Instance was started.
    start_token: std::collections::HashMap<String, serde_json::Value>,
    /// The token with which the Flow Node Instance was finished.
    end_token: Option<std::collections::HashMap<String, serde_json::Value>>,
    owner_id: String,
    error: Option<serde_json::Value>,
    started_at: Option<std::time::SystemTime>,
    finished_at: Option<std::time::SystemTime>,
    triggered_by_flow_node_instance: Option<Box<FlowNodeInstance>>,
}

/// The state of a FlowNodeInstance.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FlowNodeInstanceState {
    Running,
    Suspended,
    Finished,
    Terminated,
    Error,
    Canceled,
}

impl FlowNodeInstanceState {
    pub fn as_str(&self) -> &'static str {
        match *self {
            FlowNodeInstanceState::Running => "running",
            FlowNodeInstanceState::Suspended => "suspended",
            FlowNodeInstanceState::Finished => "finished",
            FlowNodeInstanceState::Terminated => "terminated",
            FlowNodeInstanceState::Error => "error",
            FlowNodeInstanceState::Canceled => "canceled",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProcessToken {
    flow_node_instance_id: String,
    created_at: std::time::SystemTime,
    payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FlowNodeInstancesQuery {
    pub flow_node_instance_id: Option<String>,
    pub flow_node_id: Option<String>,
    pub flow_node_name: Option<String>,
    pub flow_node_lane: Option<String>,
    pub flow_node_type: Option<String>,
    pub event_type: Option<String>,
    pub correlation_id: Option<String>,
    pub process_definition_id: Option<String>,
    pub process_model_id: Option<String>,
    pub process_instance_id: Option<String>,
    pub owner_id: Option<String>,
    pub state: Option<String>,
    pub previous_flow_node_instance_id: Option<String>,
    pub parent_process_instance_id: Option<String>,
    pub created_at: Option<std::time::SystemTime>,
    pub updated_at: Option<std::time::SystemTime>,
}

impl FlowNodeInstancesQuery {
    pub fn to_query_params(&self) -> Vec<String> {
        let mut parts = Vec::new();

        fn append_param(parts: &mut Vec<String>, key: &str, value: &Option<String>) {
            if let Some(ref val) = value {
                parts.push(format!("{}={}", key, val));
            }
        }

        fn append_timestamp(
            parts: &mut Vec<String>,
            key: &str,
            value: &Option<std::time::SystemTime>,
        ) {
            if let Some(time) = value {
                let timestamp = time
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                parts.push(format!("{}={}", key, timestamp));
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

        append_timestamp(&mut parts, "createdAt", &self.created_at);
        append_timestamp(&mut parts, "updatedAt", &self.updated_at);

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
