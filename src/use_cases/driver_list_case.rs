use crate::domain::path_generator::PathGenerator;
use crate::domain::flow_input::FlowInput;

pub struct DriverListCase {
    config_file_path: String,
    result: Vec<String>,
}

impl DriverListCase {
    // per ora instanziamo le dipendenze dentro il metodo
    // in futuro le passeremo come parametri per dependency injection
    pub fn new(config_file_path: String) -> Vec<String> {
        let absolute_config_path = PathGenerator::from_string(&config_file_path);
        let flow_input = FlowInput::from_json_file(absolute_config_path);
        flow_input.perform_flow();
        println!("Flow Input: {:?}", flow_input);
        
        
        
        todo!()
    }
}