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
    },
    ApiData {
        method: Method::POST,
        url: "https://login-mum.housing.com/api/v5/check-value-present-in-system?platform=desktop&source_name=WEB".to_owned(),
        headers: HashMap::from([
            ("Cookie".to_string(), "traffic=sourcemedium%3Dgoogle%20%2F%20organic%3B; uuid=id%3Df1eff911e6be1464785ef261c4cafb1c%3B; cuid=cb4c23e7-991e-44cb-be2f-85e7f9ac7c80; hasCuid=true; sellerExperiments=listing_tab_revamp%3Dtrue%3Bhom8403%3Dfalse%3B".to_string()),
            ("Sec-Ch-Ua".into(), "\"Not/A)Brand\";v=\"8\", \"Chromium\";v=\"126\"".into()),
            ("X-Csrf-Token-V2".into(), "009a4c37d5c1423fafa50b1943fa547514b78efc767255ff9e36ef57e3b77a69".into()),
            ("Accept-Language".into(), "en-GB".into()),
            ("Sec-Ch-Ua-Mobile".into(), "?0".into()),
            ("User-Agent".into(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.6478.127 Safari/537.36".into()),
            ("Content-Type".into(), "application/json".into()),
            ("Accept".into(), "application/json, text/plain, */*".into()),
            ("App_name".into(), "desktop_web_seller".into()),
            ("Sec-Ch-Ua-Platform".into(), "\"Linux\"".into()),
            ("Origin".into(), "https://seller.housing.com".into()),
            ("Sec-Fetch-Site".into(), "same-site".into()),
            ("Sec-Fetch-Mode".into(), "cors".into()),
            ("Sec-Fetch-Dest".into(), "empty".into()),
            ("Referer".into(), "https://seller.housing.com/".into()),
            ("Accept-Encoding".into(), "gzip, deflate, br".into()),
            ("Priority".into(), "u=1, i".into()),
        ]),
        query: Vec::new(),
        body: Some("{\"phone\":\"{target}\",\"country_url_name\":\"in\"}".to_owned()),
        identifier: "".to_owned(),
    },
    ]
}
