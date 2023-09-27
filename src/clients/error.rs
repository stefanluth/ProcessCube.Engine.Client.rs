use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents an error returned by the 5Minds Engine.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EngineError {
    #[serde(rename = "errorClassName")]
    pub error_type: String,
    pub message: String,
    #[serde(rename = "callStack")]
    pub stack: String,
    pub code: u32,
}

impl EngineError {
    pub fn new(error_type: String, message: String, stack: String, code: u32) -> EngineError {
        EngineError {
            error_type,
            message,
            stack,
            code,
        }
    }
}

impl From<reqwest::Error> for EngineError {
    /// Converts a reqwest::Error into an EngineError.
    fn from(err: reqwest::Error) -> Self {
        EngineError::new(
            "InternalError".to_string(),
            "Error processing request".to_string(),
            err.to_string(),
            500,
        )
    }
}

impl fmt::Display for EngineError {
    /// Formats an EngineError.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EngineError: error_type: {}, message: {}, stack: {}, code: {}",
            self.error_type, self.message, self.stack, self.code
        )
    }
}

impl std::error::Error for EngineError {}
