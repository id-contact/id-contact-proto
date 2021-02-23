use serde::{Serialize, Deserialize};

/// Data provided for a start_communication request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartCommRequest {
    /// Purpose of the communication session
    pub purpose: String,
    /// Attribute JWE containing attributes provided by an earlier authentication request
    pub attributes: Option<String>,
}

/// Expected result for a start_communication request
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartCommResponse {
    /// URL for client to start communication process
    pub client_url: String,
    /// URL to which authentication result will/should be sent after completion of authentication flow
    pub attr_url: Option<String>,
}
