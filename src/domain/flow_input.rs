use serde::{Deserialize, Serialize};
use crate::domain::user::User;
use crate::domain::step::{Step};
#[derive(Serialize, Deserialize, Debug)]
pub struct FlowInput {
    threads: usize,
    rampup: usize,
    intra_action_delay: usize,
    users: Vec<User>,
    flow: Vec<Step>,
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

    pub(crate) fn get_users(&self) -> &[User] { self.users.as_slice() }
    pub fn get_threads(&self) -> usize { self.threads }
    pub fn get_rampup(&self) -> usize { self.rampup }
    pub fn get_intra_action_delay(&self) -> usize { self.intra_action_delay}
    pub fn get_steps(&self) -> &[Step] { self.flow.as_slice() }
    
}