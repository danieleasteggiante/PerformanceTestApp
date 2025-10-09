use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response};
use serde_json::Value;

#[derive(Debug)]
pub(crate) struct ApiCaller {
    url: String,
    method: String,
    headers: Option<Vec<(String, String)>>,
    body: Option<Value>,
    client: Client,
}

impl ApiCaller {
    pub(crate) fn new(url: String, method: String, headers: Option<Vec<(String, String)>>, body: Option<Value>, ) -> Self { 
        ApiCaller { url, method, headers, body, client: Client::new(), }
    }
    
    pub(crate) async fn post_request(&self) -> Response {
        // String.parse es.: "test".parse() can parse into any type that implements the FromStr trait.
        let mut request = self.client.request(self.method.parse().expect("Missing method!"), &self.url);
        let headers_map = self.populate_headers_map();
        request = request.headers(headers_map);
        if let Some(ref json_body) = self.body { request = request.json(json_body); }
        request.send().await.unwrap()
    }

    fn populate_headers_map(&self) -> HeaderMap {
        if self.headers.is_none() { return HeaderMap::new(); }
        let headers_list = self.headers.as_ref().unwrap();
        headers_list.iter().fold(HeaderMap::new(), |mut map, (key, value)| {
            let name = HeaderName::from_bytes(key.as_bytes()).expect("Not valid header name");
            let val  = HeaderValue::from_str(value).expect("Not valid header value");
            map.insert(name, val);
            map
        })
    }
}
