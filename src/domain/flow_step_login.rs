use serde::Deserialize;
use serde_json::Value;
use tokio::runtime::Runtime;
use crate::domain::api_caller::ApiCaller;
use crate::domain::user::User;

#[derive(Debug, Deserialize)]
pub(crate) struct FlowStepLogin {
    endpoint: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<Value>,
}

impl FlowStepLogin {
    pub fn perform(&self, users: &[User]) {
        print!("Performing login step to endpoint: {}", self.endpoint);
        let caller = ApiCaller::new(self.endpoint.clone(), self.method.clone(), None, self.body.clone(), );
        let response = Runtime::new().unwrap().block_on(caller.post_request());

    }
}