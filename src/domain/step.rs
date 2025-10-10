use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub action: String,
    pub endpoint: String,
    pub method: String,
    pub headers: Option<Vec<(String, String)>>,
    pub body: Option<Value>,
}