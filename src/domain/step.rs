use serde::Deserialize;
use crate::domain::flow_step_login::FlowStepLogin;
use crate::domain::user::User;

pub trait Step {
    fn perform(&self, users: &[User]);
}

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum FlowStep {
    login(FlowStepLogin),
}

impl Step for FlowStep {
    fn perform(&self, users: &[User]) {
        match self {
            FlowStep::login(step) => step.perform(users),
        }
    }
}