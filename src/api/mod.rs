pub mod data;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
}

#[derive(Debug, Clone)]
pub struct ApiData {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query: Vec<(String, String)>,
    pub body: Option<String>,
    pub identifier: String,
}
impl ApiData {
    pub fn filter(self, mobile: u64) -> Self {
        let mobile = mobile.to_string();
        let re = regex::Regex::new(r"\{target\}").unwrap();
        let method = &self.method.clone();
        let url = re.replace_all(&self.url, &mobile).to_string();
        let mut query = Vec::with_capacity(self.query.capacity());
        for (k, v) in &self.query {
            query.push((k.to_string(), re.replace_all(v, &mobile).to_string()));
        }
        let body = if self.body.is_some() {
            Some(re.replace_all(&self.body.unwrap(), &mobile).to_string())
        } else {
            None
        };
        let identifier = re.replace_all(&self.identifier, &mobile).to_string();
        let mut headers: HashMap<String, String> = HashMap::with_capacity(self.headers.capacity());
        for (k, v) in self.headers {
            headers.insert(k, re.replace_all(&v, &mobile).to_string());
        }
        Self {
            method: method.clone(),
            url,
            headers,
            query,
            body,
            identifier,
        }
    }
    pub fn filter_all(data: Vec<Self>, mobile: u64) -> Vec<Self> {
        let mut n_data: Vec<Self> = Vec::with_capacity(data.capacity());
        for i in data {
            n_data.push(i.filter(mobile));
        }
        n_data
    }
}
