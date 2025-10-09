use serde::Deserialize;
use crate::domain::user::User;
use crate::domain::step::{FlowStep, Step};

#[derive(Debug, Deserialize)]
pub struct FlowInput {
    users: Vec<User>,
    flow: Vec<FlowStep>,
}

impl FlowInput {
    pub fn from_json_file(path: String) -> Self {
        let file_content = std::fs::read_to_string(path).expect("Failed to read file");
        serde_json::from_str(&file_content).expect("Failed to parse JSON")
    }
    
    /*
        // esempi di conversione tra Vec<T>, a &[T]
        let slice: &[User] = &self.users;      // Vec<T> → [T] → &[T]
        let slice2: &[User] = self.users.as_slice();
        let slice3: &[User] = &*self.users;    // esplicito: *deref Vec<T> → [T], poi & → &[T]
        differenza tra Vec e & è semplicemente che il secondo non possiede i dati ed è una gestione di RUST
    */
    pub fn perform_flow(&self) {
        self.flow.iter().for_each(|step| step.perform(self.users.as_slice()));
    }
}