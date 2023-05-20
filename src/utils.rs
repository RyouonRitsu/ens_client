use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

use serde::Deserialize;

use crate::Cli;

pub enum Mode {
    Get(HashMap<String, String>),
    Post(HashMap<String, String>),
}

impl Cli {
    /// Send a request to the server.
    pub async fn exec(&self, routing: &str, mode: Mode) -> Result<String, Box<dyn Error>> {
        match mode {
            Mode::Get(map) => {
                let mut url = String::from(&self.addr);
                url += routing;

                if !map.is_empty() {
                    let mut params = vec![];
                    map.iter().for_each(|(k, v)| {
                        params.push(format!("{}={}", k, v));
                    });
                    url += &format!("?{}", params.join("&"));
                }

                let body = reqwest::get(url).await?.text().await?;
                Ok(body)
            }
            Mode::Post(map) => {
                let client = reqwest::Client::new();
                let response = client
                    .post(format!("{}{}", &self.addr, routing))
                    .json(&map)
                    .send()
                    .await?
                    .text()
                    .await?;
                return Ok(response);
            }
        }
    }

    /// Print the response in a pretty format.
    pub fn print_response<'a, T>(response: &'a str)
    where
        T: Deserialize<'a> + Debug,
    {
        match serde_json::from_str::<T>(response) {
            Ok(t) => println!("{:#?}", t),
            Err(_) => println!("{}", response),
        }
    }
}
