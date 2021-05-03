use serde::{Deserialize, Serialize};

pub type Tag = String;
#[derive(Debug, Serialize, Deserialize)]
pub struct MethodProperties {
    pub tag: Tag,
    pub name: String,
    pub image_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionOptions {
    pub auth_methods: Vec<MethodProperties>,
    pub comm_methods: Vec<MethodProperties>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartRequestAuthOnly {
    pub purpose: String,
    pub auth_method: Tag,
    pub comm_url: String,
    pub attr_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientUrlResponse {
    pub client_url: String,
}
