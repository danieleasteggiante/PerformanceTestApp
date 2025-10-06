#[derive(Debug, Deserialize)]
struct Input {
    users: Vec<User>,
    flow: Vec<FlowStep>,
}