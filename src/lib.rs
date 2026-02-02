use std::path::Path;

use identity_common::models::user::User;
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
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub user: User
}

impl IdentityClient {
    async fn serialize_and_send_post<T>(&self, body: &T, path: &str) -> Result<Response>
    where T: Serialize
    {
        let body = serde_json::to_string(body)?;

        let path = Path::new(&self.host.clone()).join(path);
        let path = match path.to_str() {
            Some(v) => Ok(v),
            None => Err(Error::UrlParseError(anyhow::anyhow!("error parsing url"))),
        }?;

        let r = self.client.post(path).body(body).send().await?;
        Ok(r)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<Response> {
        self.serialize_and_send_post(&request, "login").await
    }

    pub async fn signup(&self, request: SignupRequest) -> Result<Response> {
        self.serialize_and_send_post(&request, "signup").await
    }

    pub async fn auth(&self, request: AuthRequest) -> Result<Response> {
        self.serialize_and_send_post(&request, "auth").await
    }
}

pub fn create_client(host: String) -> IdentityClient {
    IdentityClient { client: reqwest::Client::new(), host }
}
