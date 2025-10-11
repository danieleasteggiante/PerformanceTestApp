use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct FlowStepLogin {
    name: String,
    endpoint: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<Value>,
}