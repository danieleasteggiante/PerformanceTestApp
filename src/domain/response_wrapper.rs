use std::collections::HashMap;

pub struct ResponseWrapper { 
    email: String, 
    data: Vec<String>,
}

impl ResponseWrapper {
    
    pub fn from_email(email: String) -> Self {
        ResponseWrapper { email, data: Vec::new() }
    }
    
    pub fn add(&mut self, value: String) {
        self.data.push(format!("{}", value));
    }

}