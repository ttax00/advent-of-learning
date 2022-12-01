use std::{env, str::FromStr};

use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Url,
};
pub struct AdventAPI {
    client: Client,
    endpoint: Url,
}

impl AdventAPI {
    pub async fn get_input(&self, year: u16, day: u8) -> Result<String, Error> {
        let req = self
            .endpoint
            .join(&format!("{}/day/{}/input", year, day))
            .unwrap();
        let res = self.client.get(req).send().await?;
        Ok(res.text().await?)
    }
}

impl Default for AdventAPI {
    fn default() -> Self {
        dotenv().ok();
        let mut headers = HeaderMap::new();
        let value =
            HeaderValue::from_str(&format!("session={}", env::var("SESSION").unwrap())).unwrap();
        headers.insert("Cookie", value);
        let client = Client::builder()
            .user_agent("AoC-API")
            .default_headers(headers)
            .build()
            .unwrap();
        let endpoint = Url::from_str("https://adventofcode.com/").unwrap();
        Self { client, endpoint }
    }
}

pub async fn get_input(year: u16, day: u8) -> String {
    let client = AdventAPI::default();
    match client.get_input(year, day).await {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn dotenv_works() {
        dotenv().ok();
        env::var("SESSION").expect("SESSION env is not set.");
    }
}
