use std::collections::HashMap;
use crate::domain::step::{Step, StepMethod};

pub struct Context {
    pub(crate) data: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Context { data: HashMap::new() }
    }

    pub fn insert(&mut self, key: String, value: String) {
        if self.data.contains_key(&key) && self.data.get(&key) == Some(&value) {
            return;
        }
        self.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    pub fn update(&mut self, step: &Step, body: &String) { 
        if step == StepMethod::Login {
           self.add_bearer(body); 
        }
        self.overwrite_new_headers(step);
    }
    
    fn overwrite_new_headers(&mut self, step: &Step) { 
        let Some(headers) = &step.headers else { return; };
        headers.iter().for_each(|(k,v)| {
           self.data.insert(k.clone(), v.clone());
        });
    }
    
    fn add_bearer(&mut self, response: &String){
        if response.contains("Bearer ") {
            let token = response.split("Bearer ").nth(1).unwrap_or("").trim().to_string();
            if !token.is_empty() {
                self.data.insert("Authorization".to_string(), format!("Bearer {}", token));
            }
        }
    }
}