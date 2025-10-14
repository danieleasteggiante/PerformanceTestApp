use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct FlowStepLogin {
    name: String,
    endpoint: String,
    method: String,
    headers: HashMap<String,String>,
    body: Option<Value>,
}