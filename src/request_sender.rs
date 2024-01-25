use ansi_term::Color;
use reqwest::{self, Client};
use serde::Deserialize;
use std::error::Error;


#[derive(Debug)]
pub struct RequestSender {
    client: Client,
    ip: String,
    port: i32,
    key: String,
}

#[derive(Deserialize)]
struct Response {
    msg: String,
}

#[derive(serde::Deserialize)]
struct ShowRepoResponse {
    msg: String,
    out: Vec<String>,
}

impl RequestSender {
    pub fn new(ip: &str, port: i32, key: &str) -> RequestSender {
        RequestSender {
            client: Client::new(),
            ip: ip.to_string(),
            port: port,
            key: key.to_string(),
        }
    }

    pub fn connect(&self) -> bool {
        let runtime: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<String, Box<dyn Error>> =
            runtime.block_on(self.send_connect(&self.ip, self.port, &self.key));
        match result {
            Ok(msg) => {
                if msg == "ok" {
                    return true;
                }
            }
            Err(error) => {
                println!("{}: {}", Color::Red.paint("Connect exception"), error);
            }
        }
        false
    }

    async fn send_connect(&self, ip: &str, port: i32, key: &str) -> Result<String, Box<dyn Error>> {
        let req_url: String = format!("http://{}:{}/ping?auth={}", ip, port, key);
        let response = self.client.get(req_url).send().await?.text().await?;
        let result: Response = serde_json::from_str(&response)?;
        let msg: String = result.msg;
        Ok(msg)
    }

    pub fn show_repo(&self) -> Vec<String> {
        let runtime: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        let result: Result<ShowRepoResponse, Box<dyn Error>> = runtime.block_on(self.send_show_repo());
        match result {
            Ok(json) => {
                if  json.msg == "ok"{
                    return json.out;
                }else {
                    println!("Show repository fail");                    
                    return Vec::new();
                }
            }
            Err(e) => {
                println!("{}: {}", Color::Red.paint("Connect exception"), e);
            }
        }
        return Vec::new();
    }

    async fn send_show_repo(&self) -> Result<ShowRepoResponse, Box<dyn Error>> {
        let req_url = format!(
            "http://{}:{}/show/repo?auth={}",
            self.ip, self.port, self.key
        );
        let response = self.client.get(req_url).send().await?.text().await?;
        let res_json: ShowRepoResponse = serde_json::from_str(&response)?;
        Ok(res_json)
    }
}
