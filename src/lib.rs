use std::path::Path;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

pub use reqwest::Response;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    UrlParseError(anyhow::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
    pub token: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignupResponse {
    pub token: String
}


impl IdentityClient {
    pub async fn login(&self, request: LoginRequest) -> Result<Response> {
        let body = serde_json::to_string(&request)?;

        let path = Path::new(&self.host.clone()).join("login");
        let path = match path.to_str() {
            Some(v) => Ok(v),
            None => Err(Error::UrlParseError(anyhow::anyhow!("error parsing url"))),
        }?;

        let r = self.client.post(path).body(body).send().await?;
        Ok(r)
    }

    pub async fn signup(&self, request: SignupRequest) -> Result<Response> {
        let body = serde_json::to_string(&request)?;

        let path = Path::new(&self.host.clone()).join("signup");
        let path = match path.to_str() {
            Some(v) => Ok(v),
            None => Err(Error::UrlParseError(anyhow::anyhow!("error parsing url"))),
        }?;

        let r = self.client.post(path).body(body).send().await?;
        Ok(r)
    }
}

pub fn create_client(host: String) -> IdentityClient {
    IdentityClient { client: reqwest::Client::new(), host }
}
