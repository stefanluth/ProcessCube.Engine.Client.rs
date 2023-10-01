use crate::clients::{api::api_client::ApiClient, error::EngineError};

use super::event::EventType;

const MESSAGES_ENDPOINT: &str = "/messages";
const SIGNALS_ENDPOINT: &str = "/signals";

/// A client for triggering events in the 5Minds Engine.
#[derive(Clone)]
pub struct EventClient {
    api_client: ApiClient,
    pub messages_url: String,
    pub signals_url: String,
}

impl EventClient {
    /// Creates a new instance of the EventClient.
    ///
    /// # Arguments
    /// * `api_client` - The ApiClient to use for communication with the 5Minds Engine.
    ///
    /// # Example
    /// ```
    /// use engine_client::clients::{api::api_client::ApiClient, event::event_client::EventClient, error::EngineError};
    /// const DUMMY_TOKEN: &str = "Bearer ZHVtbXlfdG9rZW4=";
    /// const ENGINE_URL: &str = "http://localhost:10560";
    /// // Be sure to have a running 5Minds Engine at the given URL
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), EngineError> {
    ///     let api_client = ApiClient::new(ENGINE_URL, DUMMY_TOKEN);
    ///     let event_client = EventClient::new(api_client);
    ///     // Trigger a message
    ///     match event_client.trigger_message("Message_1", None, None).await {
    ///         Ok(_) => println!("Message triggered successfully"),
    ///         Err(err) => println!("Error triggering message: {}", err),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn new(api_client: ApiClient) -> EventClient {
        let messages_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            MESSAGES_ENDPOINT
        );

        let signals_url = format!(
            "{}{}{}",
            api_client.get_engine_url(),
            api_client.get_engine_api_endpoint(),
            SIGNALS_ENDPOINT
        );

        EventClient {
            api_client,
            messages_url,
            signals_url,
        }
    }

    /// Triggers an event with the given name.
    /// The event can either be a message or a signal.
    async fn trigger_event(
        &self,
        event_type: &EventType,
        event_name: &str,
        process_instance_id: Option<&str>,
        payload: Option<&str>,
    ) -> Result<(), EngineError> {
        let url_base = match event_type {
            EventType::Message => &self.messages_url,
            EventType::Signal => &self.signals_url,
        };

        let url = match process_instance_id {
            Some(process_instance_id) => format!(
                "{}/{}/trigger?processInstanceId={}",
                self.messages_url, event_name, process_instance_id
            ),
            None => format!("{}/{}/trigger", url_base, event_name),
        };

        let payload_json = match payload {
            Some(payload) => serde_json::json!({ "payload": payload }),
            None => serde_json::json!({}),
        };

        self.api_client.post::<()>(&url, Some(&payload_json)).await
    }

    /// Triggers a message with the given name.
    ///
    /// # Arguments
    /// * `message_name` - The name of the message to trigger.
    /// * `process_instance_id` - The ID of the ProcessInstance to send the message to.
    /// If this is set to `None`, the message will be sent to all ProcessInstances.
    /// * `payload` - An optional payload to send with the message.
    pub async fn trigger_message(
        &self,
        message_name: &str,
        process_instance_id: Option<&str>,
        payload: Option<&str>,
    ) -> Result<(), EngineError> {
        self.trigger_event(
            &EventType::Message,
            message_name,
            process_instance_id,
            payload,
        )
        .await
    }

    /// Triggers a signal with the given name.
    ///
    /// # Arguments
    /// * `signal_name` - The name of the signal to trigger.
    /// * `process_instance_id` - The ID of the ProcessInstance to send the signal to.
    /// If this is set to `None`, the signal will be sent to all ProcessInstances.
    /// * `payload` - An optional payload to send with the signal.
    pub async fn trigger_signal(
        &self,
        signal_name: &str,
        process_instance_id: Option<&str>,
        payload: Option<&str>,
    ) -> Result<(), EngineError> {
        self.trigger_event(
            &EventType::Signal,
            signal_name,
            process_instance_id,
            payload,
        )
        .await
    }
}
