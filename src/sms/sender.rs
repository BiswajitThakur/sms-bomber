use core::{panic, str};
use std::{thread, time::Duration};

use crate::api::{ApiData, Method};
use crate::cli::{App, Limit};

pub struct Attacker {
    data: Vec<ApiData>,
    app: App,
}

impl Attacker {
    pub fn init(data: Vec<ApiData>, app: App) -> Self {
        Self { data, app }
    }
}

impl Attacker {
    pub fn attack(&self) {
        match self.app.get_limit() {
            Limit::Infinity => infinity_attack(self),
            Limit::Value(_) => {
                panic!("This features is not available on this version.")
            }
        }
    }
}
fn infinity_attack(atkr: &Attacker) {
    let mut i: usize = 0;
    let mut success_count: usize = 0;
    let mut faild_count: usize = 0;
    if !atkr.app.no_output {
        println!("<ctrl>+c to close....");
    };
    loop {
        let r = attack(&atkr.data[i], atkr.app.no_output);
        if r.is_ok() && r.unwrap() {
            success_count += 1;
        } else {
            faild_count += 1;
        };
        i = (i + 1) % atkr.data.len();
        if !atkr.app.no_output {
            println!("Success: {}\t Faild: {}", success_count, faild_count);
        };
        thread::sleep(Duration::from_secs(atkr.app.delay))
    }
}
fn attack(data: &ApiData, no_output: bool) -> Result<bool, ureq::Error> {
    match &data.method {
        Method::POST => {
            let mut resp = ureq::post(&data.url);
            for (k, v) in &data.headers {
                resp = resp.set(k, v);
            }
            let mut query: Vec<(&str, &str)> = Vec::with_capacity(data.query.capacity());
            for (i, j) in &data.query {
                query.push((i.as_str(), j.as_str()));
            }
            resp = resp.query_pairs(query);
            if let Some(body) = &data.body {
                let body_bytes = body.as_bytes();
                resp = resp.set("Content-Length", body_bytes.len().to_string().as_str());
                let r = resp.send_bytes(body_bytes)?.into_string()?;
                if !no_output && r.find(&data.identifier).is_some() {
                    return Ok(true);
                };
            } else {
                let r = resp.call()?.into_string()?;
                if !no_output && r.find(&data.identifier).is_some() {
                    return Ok(true);
                };
            }
        }
        Method::GET => {
            let mut resp = ureq::get(&data.url);
            for (k, v) in &data.headers {
                resp = resp.set(k, v);
            }
            let mut query: Vec<(&str, &str)> = Vec::with_capacity(data.query.capacity());
            for (i, j) in &data.query {
                query.push((i.as_str(), j.as_str()));
            }
            let r = resp.call()?.into_string()?;
            if !no_output && r.contains(&data.identifier) {
                return Ok(true);
            };
        }
        _ => {
            panic!("Not implemented yet")
        }
    }
    Ok(false)
}
