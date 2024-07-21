use super::ApiData;

fn get_data(code: u32) -> Option<String> {
    match code {
        91 => Some(include_str!("./../../data/91.txt").to_owned()),
        _ => None,
    }
}

fn get_global() -> String {
    let g = include_str!("./../../data/global.txt");
    g.trim().to_string()
}

pub fn string_to_vec_api_data(s: String) -> Vec<ApiData> {
    if s.is_empty() {
        return Vec::new();
    };
    let splitter = "++++++++++";
    let vals: Vec<&str> = s.as_str().split(splitter).collect();
    let mut data: Vec<ApiData> = Vec::with_capacity(vals.len());
    for i in vals {
        let i = i.trim();
        if !i.is_empty() {
            data.push(ApiData::from(i));
        };
    }
    data
}

pub fn get(code: u32) -> Vec<ApiData> {
    let d1 = get_data(code);
    let mut d2: Vec<ApiData> = if d1.is_some() {
        string_to_vec_api_data(d1.unwrap())
    } else {
        Vec::new()
    };
    d2.extend(string_to_vec_api_data(get_global()));
    d2
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::api::Method;

    use super::*;

    #[test]
    fn test_string_to_vec_api_data_0() {
        let input = "".to_owned();
        let want = Vec::new();
        let get = string_to_vec_api_data(input);
        assert_eq!(get, want);
    }
    #[test]
    fn test_string_to_vec_api_data_1() {
        let input = "  \n  \n  ".to_owned();
        let want = Vec::new();
        let get = string_to_vec_api_data(input);
        assert_eq!(get, want);
    }

    #[test]
    #[should_panic]
    fn test_string_to_vec_api_data_2() {
        let input = "invalid data".to_owned();
        let _ = string_to_vec_api_data(input);
    }
    #[test]
    fn test_string_to_vec_api_data_3() {
        let input = r#"
POST /api/7/user/otp/generate HTTP/1.1
Host: 2.rome.api.example.com
Cookie: T=clytsxmsw0czo13eb4701u5pr-BR1721460162992; at=eyJO-WvLxhMaSU
Content-Length: 27
Sec-Ch-Ua: "Not/A)Brand";v="8", "Chromium";v="126"
identifier: "ok"

{"this":"+is+body"}

++++++++++

POST /api/abc/xyz HTTP/1.1
Host: api.example.com
Accept: */*
Priority: u=1, i
Connection: keep-alive
identifier: "hello"

++++++++++
GET /example/abc/xyz HTTP/1.1
Host: example.com
Accept: */*
Priority: u=1, i
Connection: keep-alive
identifier: testing

            "#;
        let want = Vec::from([
            ApiData {
                method: Method::POST,
                url: "https://2.rome.api.example.com/api/7/user/otp/generate".to_owned(),
                headers: HashMap::from([
                    ("Host".to_owned(), "2.rome.api.example.com".to_owned()),
                    (
                        "Cookie".to_owned(),
                        "T=clytsxmsw0czo13eb4701u5pr-BR1721460162992; at=eyJO-WvLxhMaSU".to_owned(),
                    ),
                    ("Content-Length".to_owned(), "27".to_owned()),
                    (
                        "Sec-Ch-Ua".to_owned(),
                        "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\"".to_owned(),
                    ),
                ]),
                query: Vec::new(),
                body: Some("{\"this\":\"+is+body\"}".to_owned()),
                identifier: "\"ok\"".to_owned(),
            },
            ApiData {
                method: Method::POST,
                url: "https://api.example.com/api/abc/xyz".to_owned(),
                headers: HashMap::from([
                    ("Host".to_owned(), "api.example.com".to_owned()),
                    ("Accept".to_owned(), "*/*".to_owned()),
                    ("Priority".to_owned(), "u=1, i".to_owned()),
                    ("Connection".to_owned(), "keep-alive".to_owned()),
                ]),
                query: Vec::new(),
                body: None,
                identifier: "\"hello\"".to_owned(),
            },
            ApiData {
                method: Method::GET,
                url: "https://example.com/example/abc/xyz".to_owned(),
                headers: HashMap::from([
                    ("Host".to_owned(), "example.com".to_owned()),
                    ("Accept".to_owned(), "*/*".to_owned()),
                    ("Priority".to_owned(), "u=1, i".to_owned()),
                    ("Connection".to_owned(), "keep-alive".to_owned()),
                ]),
                query: Vec::new(),
                body: None,
                identifier: "testing".to_owned(),
            },
        ]);
        let get = string_to_vec_api_data(input.to_owned());
        assert_eq!(get, want);
    }
}
