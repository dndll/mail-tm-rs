use anyhow::Error;
use reqwest::{Client as ReqwestClient, StatusCode};
use reqwest::ClientBuilder;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT as USER_AGENT_PARAM};

use crate::error::HttpError;
use crate::USER_AGENT;

pub struct Client {
    headers: HeaderMap<HeaderValue>,
    builder: ClientBuilder,
}

impl Client {
    pub fn new() -> Result<Client, Error> {
        let client = Client { // TODO: This can be cached
            headers: get_headers()?,
            builder: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .referer(true),
        };
        Ok(client)
    }

    pub fn with_auth(mut self, token: &str) -> Result<Client, Error> {
        self.headers
            .insert(AUTHORIZATION, format!("Bearer {}", token).parse()?);
        Ok(self)
    }

    pub fn build(self) -> Result<ReqwestClient, Error> {
        Ok(self.builder.default_headers(self.headers).build()?)
    }
}

pub fn get_headers() -> Result<HeaderMap<HeaderValue>, Error> {
    let mut header_map = HeaderMap::new();
    header_map.insert(USER_AGENT_PARAM, USER_AGENT.parse()?);
    header_map.insert("Origin", "https://mail.tm".parse()?); // TODO test if needed
    header_map.insert("TE", "Trailers".parse()?); // TODO test if needed
    header_map.insert(CONTENT_TYPE, "application/json;charset=utf-8".parse()?); //TODO memoize me
    Ok(header_map)
}

pub async fn check_response_status(status: &StatusCode, res: &str) -> Result<(), Error> {
    if !status.is_success() {
        return Err(HttpError::Status(status.as_u16(), res.to_string()).into());
    }
    Ok(())
}
