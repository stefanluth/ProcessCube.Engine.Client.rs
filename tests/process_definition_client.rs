use engine_client::clients::{
    client_factory::ClientFactory,
    process_definition::process_definition::PersistProcessDefinitionPayload,
};

const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
const ENGINE_URL: &str = "http://localhost:10560";
const VALID_PROCESS_DEFINITION_XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?> <bpmn:definitions xmlns:bpmn=\"http://www.omg.org/spec/BPMN/20100524/MODEL\" xmlns:bpmndi=\"http://www.omg.org/spec/BPMN/20100524/DI\" xmlns:dc=\"http://www.omg.org/spec/DD/20100524/DC\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:di=\"http://www.omg.org/spec/DD/20100524/DI\" id=\"g0de3a461-017c-4942-82b1-c32e9dd0f85d_Definition\" targetNamespace=\"http://bpmn.io/schema/bpmn\" exporter=\"5Minds Studio\" exporterVersion=\"1\"> <bpmn:collaboration id=\"Collaboration_1cidyxu\" name=\"\"> <bpmn:participant id=\"Participant_0px403d\" name=\"Foo Process\" processRef=\"g6a46d5ff-c231-4b2c-b335-e28bc440fc18_Process\" /> </bpmn:collaboration> <bpmn:process id=\"g6a46d5ff-c231-4b2c-b335-e28bc440fc18_Process\" name=\"Foo Process\" isExecutable=\"true\"> <bpmn:laneSet> <bpmn:lane id=\"Lane_1xzf0d3\" name=\"Bar Lane\"> <bpmn:flowNodeRef>StartEvent_1</bpmn:flowNodeRef> <bpmn:flowNodeRef>Event_1lpuf0o</bpmn:flowNodeRef> </bpmn:lane> </bpmn:laneSet> <bpmn:startEvent id=\"StartEvent_1\" name=\"Start\"> <bpmn:outgoing>Flow_1t9bzyi</bpmn:outgoing> </bpmn:startEvent> <bpmn:sequenceFlow id=\"Flow_1t9bzyi\" sourceRef=\"StartEvent_1\" targetRef=\"Event_1lpuf0o\" /> <bpmn:endEvent id=\"Event_1lpuf0o\" name=\"End\"> <bpmn:incoming>Flow_1t9bzyi</bpmn:incoming> </bpmn:endEvent> </bpmn:process> <bpmndi:BPMNDiagram id=\"BPMNDiagram_1\"> <bpmndi:BPMNPlane id=\"BPMNPlane_1\" bpmnElement=\"Collaboration_1cidyxu\"> <bpmndi:BPMNShape id=\"Participant_0px403d_di\" bpmnElement=\"Participant_0px403d\" isHorizontal=\"true\"> <dc:Bounds x=\"5\" y=\"100\" width=\"335\" height=\"150\" /> <bpmndi:BPMNLabel /> </bpmndi:BPMNShape> <bpmndi:BPMNShape id=\"Lane_1xzf0d3_di\" bpmnElement=\"Lane_1xzf0d3\" isHorizontal=\"true\"> <dc:Bounds x=\"35\" y=\"100\" width=\"305\" height=\"150\" /> <bpmndi:BPMNLabel /> </bpmndi:BPMNShape> <bpmndi:BPMNShape id=\"StartEvent_1_di\" bpmnElement=\"StartEvent_1\"> <dc:Bounds x=\"92\" y=\"152\" width=\"36\" height=\"36\" /> <bpmndi:BPMNLabel> <dc:Bounds x=\"98\" y=\"195\" width=\"24\" height=\"14\" /> </bpmndi:BPMNLabel> </bpmndi:BPMNShape> <bpmndi:BPMNShape id=\"Event_1lpuf0o_di\" bpmnElement=\"Event_1lpuf0o\"> <dc:Bounds x=\"252\" y=\"152\" width=\"36\" height=\"36\" /> <bpmndi:BPMNLabel> <dc:Bounds x=\"260\" y=\"195\" width=\"20\" height=\"14\" /> </bpmndi:BPMNLabel> </bpmndi:BPMNShape> <bpmndi:BPMNEdge id=\"Flow_1t9bzyi_di\" bpmnElement=\"Flow_1t9bzyi\"> <di:waypoint x=\"128\" y=\"170\" /> <di:waypoint x=\"252\" y=\"170\" /> </bpmndi:BPMNEdge> </bpmndi:BPMNPlane> </bpmndi:BPMNDiagram> </bpmn:definitions>";

#[tokio::test]
async fn upload_process_definition() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_process_definition_client();
    let request = PersistProcessDefinitionPayload {
        xml: VALID_PROCESS_DEFINITION_XML.to_string(),
        overwrite_existing: true,
    };
    let result = client.upload_process_definition(request).await;

    match result {
        Ok(_) => (),
        Err(e) => panic!("Error uploading process definition: {:#?}", e),
    }
}

#[tokio::test]
async fn get_process_definitions() {
    let client_factory = ClientFactory::new(ENGINE_URL, DUMMY_TOKEN);
    let client = client_factory.create_process_definition_client();
    let result = client.get_process_definitions(None, None).await;

    match result {
        Ok(_) => (),
        Err(e) => panic!("Error getting process definitions: {:#?}", e),
    }
}
