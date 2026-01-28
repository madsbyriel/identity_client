use reqwest::Client;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error)
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJson(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Reqwest(value)
    }
}

pub struct IdentityClient {
    client: Client,
    host: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}


impl IdentityClient {
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        let body = serde_json::to_string(&request)?;

        let r = self.client.post(self.host.clone()).body(body).send().await?;
        let r = r.text().await?;
        let r = serde_json::from_str::<LoginResponse>(r.as_str())?;

        Ok(r)
    }
}

pub fn create_client(host: String) -> IdentityClient {
    IdentityClient { client: reqwest::Client::new(), host }
}

