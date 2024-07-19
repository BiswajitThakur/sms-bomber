use std::collections::HashMap;

use super::ApiData;
use super::Method;

pub fn get() -> Vec<ApiData> {
    vec![ApiData {
        method: Method::POST,
        url: "https://www.jio.com/api/jio-login-service/login/sendOtp".to_string(),
        headers: HashMap::from([
            ("Cookie".to_string(),"JioSessionID=8c7ef27e-5a29-4bb6-af64-f7ea010a2adb; ssjsid=8c7ef27e-5a29-4bb6-af64-f7ea010a2adb; SITESJSESSIONID=ZkW-8pqJmnqLNrmnB-Q1XaNcBG3bzR8NKAocQ1t0tHYNoNpx9u3t!-1783398066!-33467015; ADRUM_BTa=R:35|g:7ce7020b-26c5-4135-8440-b8bf138879b0|n:customer1_a309c9d0-b5ef-4ff1-8978-610c0b29df8f; SameSite=None; NSC_JOwbp5gbcg11ql1emgwtjreciym2vb3=ffffffff0985b18645525d5f4f58455e445a4a4229a0".to_string()),
            ("Sec-Ch-Ua".to_owned(), "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\"".to_owned()),
            ("Sec-Ch-Ua-Platform".to_owned(), "\"Linux\"".to_owned()),
            ("Accept-Language".to_owned(), "en-GB".to_owned()),
            ("Sec-Ch-Ua-Mobile".to_string(), "?0".to_string()),
            ("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Accept".to_string(), "*/*".to_owned()),
            ("Origin".to_owned(), "https://www.jio.com".to_owned()),
            ("Sec-Fetch-Site".to_owned(), "same-origin".to_owned()),
            ("Sec-Fetch-Mode".to_owned(), "cors".to_owned()),
            ("Sec-Fetch-Dest".to_owned(), "empty".to_owned()),
            ("Referer".to_owned(), "https://www.jio.com/selfcare/login/".to_owned()),
            ("Accept-Encoding".to_owned(), "gzip, deflate, br".to_owned()),
            ("Priority".to_owned(), "u=1, i".to_owned()),
            ("Connection".to_string(), "keep-alive".to_owned()),
        ]),
        query: Vec::new(),
        body: Some("{\"mobileNumber\":\"{target}\",\"loginFlowType\":\"MOBILE\",\"alternateNumber\":\"\"}".to_owned()),
        identifier: "SUCCESS".to_string(),
    }]
}
