use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{get, Client, Response};
use serde_json::Value;
use crate::domain::context::Context;
use crate::domain::step::Step;
use crate::domain::user::User;

#[derive(Debug)]
pub(crate) struct ApiCaller {
}

impl ApiCaller {
    pub(crate) async fn call(step: &Step, context: &Context, user: &User) -> Response {
        let client = Client::new();
        let url = step.endpoint.replace("{email}", &user.email);
        let mut request = client.request(step.method.to_string().parse().expect("Missing method!"), url);
        let headers_map = ApiCaller::populate_headers_map(context, step);
        request = request.headers(headers_map);
        request.send().await.expect("Errore durante la richiesta HTTP Login")
    }

    fn populate_headers_map(context: &Context, step: &Step) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(step_headers) = &step.headers {
            ApiCaller::add_headers(&mut headers, step_headers);
        }
        ApiCaller::add_headers(&mut headers, &context.data);
        headers
    }

    fn add_headers(headers: &mut HeaderMap, source: &HashMap<String,String>) {
      source.iter().for_each(|(k,v)| {
          if k.is_empty() || v.is_empty() { return; }
          let header_name = HeaderName::from_bytes(k.as_bytes()).expect("Invalid header name");
          let header_value = HeaderValue::from_str(v).expect("Invalid header value");
          headers.insert(header_name, header_value);
      })
    }
    
}
