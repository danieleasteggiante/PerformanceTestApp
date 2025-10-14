use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub action: String,
    pub endpoint: String,
    pub method: StepMethod,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum StepMethod {
    Login,
    Post,
    Get,
}

impl Display for StepMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StepMethod::Login => write!(f, "login"),
            StepMethod::Post => write!(f, "post"),
            StepMethod::Get => write!(f, "get"),
        }
    }
}


impl PartialEq<StepMethod> for &Step {
    fn eq(&self, other: &StepMethod) -> bool {
        &self.action.to_string() == &other.to_string()
    }
}