use std::collections::HashMap;
use std::sync::Arc;
use futures::future::join_all;
use reqwest::Response;
use tokio::sync::Mutex;
use crate::domain::api_caller::ApiCaller;
use crate::domain::path_generator::PathGenerator;
use crate::domain::flow_input::FlowInput;

pub struct DriverListCase {
    flow_input: FlowInput
}

impl DriverListCase {
    // per ora instanziamo le dipendenze dentro il metodo
    // in futuro le passeremo come parametri per dependency injection
    pub fn new(config_file_path: String) -> Self {
        let absolute_config_path = PathGenerator::from_string(&config_file_path);
        let flow_input = FlowInput::from_json_file(absolute_config_path);
        DriverListCase { flow_input }
    }
    pub async fn execute(&self) {
        let mail_token_map: HashMap<String, String> = self.perform_login().await;
        //let result_drivers = self.perform_driver_list(mail_token_map).await;
        //println!("Result drivers: {:#?}", result_drivers);
    }
    
    async fn perform_driver_list(&self, mail_token_map: HashMap<String, String>) -> Vec<Response> {
        let step = self.flow_input.get_step_by_name("driver_list").expect("Missing step by driver_list!");
        let tasks = mail_token_map.iter()
            .map(|(email, token)| {
                let api_caller = ApiCaller::new(step.endpoint.clone().replace("{}",email), step.method.clone(), step.headers.clone(), step.body.clone());
                async move {
                    let resp = api_caller.get_request().await;
                    println!("API caller returned: {:#?}", resp);
                    resp
                }
            });
        let results = join_all(tasks).await;
        results
    }

    async fn perform_login(&self) -> HashMap<String, String> {
        let step = self.flow_input.get_step_by_name("login").expect("Missing step by login!");
        let tasks = self.flow_input.get_users().iter()
            .map(|user| {
                let api_caller = ApiCaller::new(step.endpoint.clone().replace("{email}",user.email.as_str()), step.method.clone(), step.headers.clone(), step.body.clone());
                let email = user.email.clone();
                async move {
                    let resp = api_caller.get_request().await;
                    let token = resp.text().await.unwrap();
                    (email, token)
                }
            });
        let results = join_all(tasks).await;
        results.into_iter().collect()
    }
}