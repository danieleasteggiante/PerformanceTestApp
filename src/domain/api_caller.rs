use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::blocking::{Client, RequestBuilder, Response};

use crate::domain::context::Context;
use crate::domain::step::Step;
use crate::domain::user::User;

#[derive(Debug)]
pub(crate) struct ApiCaller {}

impl ApiCaller {
    pub(crate) fn call(step: &Step, context: &Context, user: &User) -> Response {
        let request = Self::compose_request_builder(step, context, user);
        Self::perform_request(user, request)
    }

    fn compose_request_builder(step: &Step, context: &Context, user: &User) -> RequestBuilder {
        let client = Client::builder()
            .timeout(Duration::from_secs(step.max_timeout.unwrap_or(1000)))
            .build().expect("Error while creation HTTP Client");
        let url = step.endpoint.replace("{email}", &user.email);
        let mut request = client.request(step.method.to_string().parse().expect("Missing method!"), url);
        let headers_map = ApiCaller::populate_headers_map(context, step);
        request = request.headers(headers_map);
        request
    }

    fn perform_request(user: &User, request: RequestBuilder) -> Response {
        let response = request.send();
        match response {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error during API call for user {}: {}", user.email, e);
                panic!("Error during API call for user {}: {}", user.email, e);
            },
        }
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
