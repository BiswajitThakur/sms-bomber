pub mod country;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
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

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        let m: Vec<&str> = value.trim().split(' ').collect();
        match m.get(0).unwrap().to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "PATCH" => Self::PATCH,
            "HEAD" => Self::HEAD,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            _ => panic!("Invalid method..."),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiData {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query: Vec<(String, String)>,
    pub body: Option<String>,
    pub identifier: String,
}
impl ApiData {
    pub fn filter(self, code: u32, mobile: u64) -> Self {
        let mobile = mobile.to_string();
        let code = code.to_string();
        let re_mob = regex::Regex::new(r"\{\{target\}\}").unwrap();
        let re_code = regex::Regex::new(r"\{\{code\}\}").unwrap();
        let method = &self.method.clone();
        let url = re_mob.replace_all(&self.url, &mobile).to_string();
        let url = re_code.replace_all(&url, &code).to_string();
        let mut query = Vec::with_capacity(self.query.capacity());
        for (k, v) in &self.query {
            let v = re_mob.replace_all(v, &mobile).to_string();
            let v = re_code.replace_all(&v, &mobile).to_string();
            query.push((k.to_string(), v));
        }
        let body = if self.body.is_some() {
            let b = re_mob.replace_all(&self.body.unwrap(), &mobile).to_string();
            let b = re_code.replace_all(&b, &code).to_string();
            Some(b)
        } else {
            None
        };
        let identifier = re_mob.replace_all(&self.identifier, &mobile).to_string();
        let identifier = re_code.replace_all(&identifier, &code).to_string();
        let mut headers: HashMap<String, String> = HashMap::with_capacity(self.headers.capacity());
        for (k, v) in self.headers {
            let v = re_mob.replace_all(&v, &mobile).to_string();
            let v = re_code.replace_all(&v, &code).to_string();
            headers.insert(k, v);
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
    pub fn filter_all(data: Vec<Self>, code: u32, mobile: u64) -> Vec<Self> {
        let mut n_data: Vec<Self> = Vec::with_capacity(data.capacity());
        for i in data {
            n_data.push(i.filter(code, mobile));
        }
        n_data
    }
}

impl From<&str> for ApiData {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let first_line = lines.next().unwrap().trim();
        let method = Method::from(first_line);
        //let re = Regex::new(r"^\s*([^:\s]+)\s*:\s*([^\n]+)$").unwrap();
        let mut is_body = false;
        let mut kv = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                is_body = true;
                continue;
            };
            if is_body && !line.is_empty() {
                kv.insert("body", line);
                break;
            };
            let split_at_colon = line.split_once(':');
            if split_at_colon.is_none() {
                continue;
            };
            let (k, v) = split_at_colon.unwrap();
            kv.insert(k.trim(), v.trim());
        }
        let url = format!(
            "https://{}{}",
            kv.get(&"Host").unwrap(),
            first_line.split(' ').collect::<Vec<&str>>().get(1).unwrap()
        );
        let body = if kv.get(&"body").is_some() {
            Some(kv.remove(&"body").unwrap().to_owned())
        } else {
            None
        };
        let identifier = kv.remove(&"identifier").unwrap().to_owned();
        let mut headers: HashMap<String, String> = HashMap::with_capacity(kv.len());
        for (k, v) in kv {
            headers.insert(k.to_owned(), v.to_owned());
        }
        Self {
            method,
            url,
            headers,
            query: Vec::new(),
            body,
            identifier,
        }
    }
}

/*

*/
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ApiData, Method};

    #[test]
    fn test_api_data_from_str() {
        let want = ApiData {
            method: Method::POST,
            url: "https://2.rome.api.example.com/api/7/user/otp/generate".to_owned(),
            headers: HashMap::from([
                ("Host".to_owned(), "2.rome.api.example.com".to_owned()),
                ("Cookie".to_owned(), "T=clytsxmsw0czo13eb4701u5pr-BR1721460162992; at=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjFkOTYzYzUwLTM0YjctNDA1OC1iMTNmLWY2NDhiODFjYTBkYSJ9.eyJleHAiOjE3MjMxODgxNjMsImlhdCI6MTcyMTQ2MDE2MywiaXNzIjoia2V2bGFyIiwianRpIjoiOGIwNzAzOGQtZDEzNi00NmJhLWFmZGEtYjI1ZjA0ZGQwZGFkIiwidHlwZSI6IkFUIiwiZElkIjoiY2x5dHN4bXN3MGN6bzEzZWI0NzAxdTVwci1CUjE3MjE0NjAxNjI5OTIiLCJrZXZJZCI6IlZJNzA0NjUzRUNEM0VENEFFNjgwRjk5QkQyREVDOUYyQjEiLCJ0SWQiOiJtYXBpIiwidnMiOiJMTyIsInoiOiJIWUQiLCJtIjp0cnVlLCJnZW4iOjR9.KLr0N1qU9v3i02Awoo2sdm-2AQidzOfMsCWvLxhMaSU; K-ACTION=null; vh=902; vw=1143; dpr=1; Network-Type=4g; _pxvid=dbdea33a-4668-11ef-8f16-5ce015e807fe; gpv_pn=HomePage; gpv_pn_t=FLIPKART%3AHomePage; AMCVS_17EB401053DAF4840A490D4C%40AdobeOrg=1; SN=VI704653ECD3ED4AE680F99BD2DEC9F2B1.TOK37F0B4301F6B4D6E90F4E36B26C11BB7.1721460176.LO; rt=null; ud=5.C-qjxZfTgyj7ggjhZvArzoBhiDLOcLLJccB94sOzGCDCgckkEYyIFlxgVB98FVTNm3X3eGORImWu41IucuZbsJyEOQW3dFpHBNpqJp2YAxUgVBS3DYsCvb8j6B3vfloP_pjFNsVm2PljHBJLRDH_Xw; AMCV_17EB401053DAF4840A490D4C%40AdobeOrg=-227196251%7CMCIDTS%7C19925%7CMCMID%7C86226760952171595691313691549506144953%7CMCAID%7CNONE%7CMCOPTOUT-1721467385s%7CNONE; s_sq=flipkart-prd%3D%2526pid%253Dwww.flipkart.com%25253Aaccount%25253Alogin%2526pidt%253D1%2526oid%253Dfunctionkr%252528%252529%25257B%25257D%2526oidt%253D2%2526ot%253DSPAN; vd=VI704653ECD3ED4AE680F99BD2DEC9F2B1-1721460176761-1.1721460682.1721460176.156424118; S=d1t10Pyg/TyE/Pz8/fj8/Pz8/P3edrrtV6c/+dkdLBK9gOtmxyQAiayvTHJxn0uFGueIRstS0VbXE+w0vBFNLUNjCfQ==".to_owned()),
                ("Sec-Ch-Ua".to_owned(), "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\"".to_owned()),
                ("X-User-Agent".to_owned(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36 FKUA/website/42/website/Desktop".to_owned()),
                ("Content-Type".to_owned(), "application/json".to_owned()),
                ("Accept-Language".to_owned(), "en-GB".to_owned()),
                ("Sec-Ch-Ua-Mobile".to_owned(), "?0".to_owned()),
                ("User-Agent".to_owned(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36".to_owned()),
                ("Sec-Ch-Ua-Platform".to_owned(), "\"Linux\"".to_owned()),
                ("Accept".to_owned(), "*/*".to_owned()),
                ("Origin".to_owned(), "https://www.example.com".to_owned()),
                ("Accept-Encoding".to_owned(), "gzip, deflate, br".to_owned()),
                ("Priority".to_owned(), "u=1, i".to_owned()),
                ("Connection".to_owned(), "keep-alive".to_owned()),
            ]),
            query: Vec::new(),
            body: Some("{\"loginId\":\"+91{target}\"}".to_owned()),
            identifier: "...success...".to_owned(),
        };
        let got = ApiData::from(
            r#"POST /api/7/user/otp/generate HTTP/1.1
Host: 2.rome.api.example.com
Cookie: T=clytsxmsw0czo13eb4701u5pr-BR1721460162992; at=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjFkOTYzYzUwLTM0YjctNDA1OC1iMTNmLWY2NDhiODFjYTBkYSJ9.eyJleHAiOjE3MjMxODgxNjMsImlhdCI6MTcyMTQ2MDE2MywiaXNzIjoia2V2bGFyIiwianRpIjoiOGIwNzAzOGQtZDEzNi00NmJhLWFmZGEtYjI1ZjA0ZGQwZGFkIiwidHlwZSI6IkFUIiwiZElkIjoiY2x5dHN4bXN3MGN6bzEzZWI0NzAxdTVwci1CUjE3MjE0NjAxNjI5OTIiLCJrZXZJZCI6IlZJNzA0NjUzRUNEM0VENEFFNjgwRjk5QkQyREVDOUYyQjEiLCJ0SWQiOiJtYXBpIiwidnMiOiJMTyIsInoiOiJIWUQiLCJtIjp0cnVlLCJnZW4iOjR9.KLr0N1qU9v3i02Awoo2sdm-2AQidzOfMsCWvLxhMaSU; K-ACTION=null; vh=902; vw=1143; dpr=1; Network-Type=4g; _pxvid=dbdea33a-4668-11ef-8f16-5ce015e807fe; gpv_pn=HomePage; gpv_pn_t=FLIPKART%3AHomePage; AMCVS_17EB401053DAF4840A490D4C%40AdobeOrg=1; SN=VI704653ECD3ED4AE680F99BD2DEC9F2B1.TOK37F0B4301F6B4D6E90F4E36B26C11BB7.1721460176.LO; rt=null; ud=5.C-qjxZfTgyj7ggjhZvArzoBhiDLOcLLJccB94sOzGCDCgckkEYyIFlxgVB98FVTNm3X3eGORImWu41IucuZbsJyEOQW3dFpHBNpqJp2YAxUgVBS3DYsCvb8j6B3vfloP_pjFNsVm2PljHBJLRDH_Xw; AMCV_17EB401053DAF4840A490D4C%40AdobeOrg=-227196251%7CMCIDTS%7C19925%7CMCMID%7C86226760952171595691313691549506144953%7CMCAID%7CNONE%7CMCOPTOUT-1721467385s%7CNONE; s_sq=flipkart-prd%3D%2526pid%253Dwww.flipkart.com%25253Aaccount%25253Alogin%2526pidt%253D1%2526oid%253Dfunctionkr%252528%252529%25257B%25257D%2526oidt%253D2%2526ot%253DSPAN; vd=VI704653ECD3ED4AE680F99BD2DEC9F2B1-1721460176761-1.1721460682.1721460176.156424118; S=d1t10Pyg/TyE/Pz8/fj8/Pz8/P3edrrtV6c/+dkdLBK9gOtmxyQAiayvTHJxn0uFGueIRstS0VbXE+w0vBFNLUNjCfQ==
Sec-Ch-Ua: "Not/A)Brand";v="8", "Chromium";v="126"
X-User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36 FKUA/website/42/website/Desktop
Content-Type: application/json
Accept-Language: en-GB
Sec-Ch-Ua-Mobile: ?0
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36
Sec-Ch-Ua-Platform: "Linux"
Accept: */*
Origin: https://www.example.com
Accept-Encoding: gzip, deflate, br
Priority: u=1, i
Connection: keep-alive
identifier: ...success...

{"loginId":"+91{target}"}"#,
        );
        assert_eq!(got, want);
    }
}
