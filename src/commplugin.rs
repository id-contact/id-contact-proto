use serde::{Serialize, Deserialize};

/// Data provided for a start_communication request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartCommRequest {
    /// Purpose of the communication session
    pub purpose: String,
    /// JWT containing authentication session results provided by an earlier authentication request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_result: Option<String>,
}

/// Expected result for a start_communication request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartCommResponse {
    /// URL for client to start communication process
    pub client_url: String,
    /// URL to which authentication result will/should be sent after completion of authentication flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_url: Option<String>,
}
