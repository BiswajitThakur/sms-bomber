use crate::{cli::Limit, sms::api_data::Method};

use super::api_data::ApiData;

use ureq::Request;

pub fn pst() {
    let body: String = ureq::get("https://github.com/algesten/ureq?tab=readme-ov-file")
        .set("Example-Header", "header value")
        .call()
        .unwrap()
        .into_string()
        .unwrap();
    println!("{}", body);
}

#[allow(unused)]
pub struct Attacker {
    data: Vec<ApiData>,
    limit: Limit,
}

impl Attacker {
    pub fn attack(data: &ApiData, show_output: bool) -> Result<bool, ureq::Error> {
        match &data.method {
            Method::POST => {
                let mut resp = ureq::post(&data.url);
                for (k, v) in &data.headers {
                    resp = resp.set(k, v);
                }
                let resp = resp.call()?.into_string()?;
                println!("{}", resp);
            }
            _ => {}
        }
        todo!()
    }
}

impl Attacker {
    pub fn random_attack(&self) -> Result<bool, ureq::Error> {
        todo!()
    }
    pub fn index_attack(&self, index: usize) -> Result<(), ureq::Error> {
        todo!()
    }
    pub fn sequence_attack(&self) -> Result<(), ureq::Error> {
        todo!()
    }
}
