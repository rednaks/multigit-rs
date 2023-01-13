pub mod branches;
pub mod commits;
mod github;
pub mod pulls;
pub mod references;
pub mod repos;
mod response;
pub mod teams;
pub mod users;

pub use github::Github;

use std::collections::HashMap;

use reqwest::{header, RequestBuilder, Response};

pub trait GithubAPIError {
    fn error_message(&self) -> String;
    fn extra_info(&self) -> Option<String>;
}

pub struct GithubAPIResponseDeserializeError {
    pub parse_error: String,
    pub original_response: Option<String>,
}

pub struct GithubAPIResponseError {
    pub message: String,
}

impl GithubAPIError for GithubAPIResponseError {
    fn error_message(&self) -> String {
        self.message.clone()
    }

    fn extra_info(&self) -> Option<String> {
        None
    }
}

impl GithubAPIError for GithubAPIResponseDeserializeError {
    fn error_message(&self) -> String {
        self.parse_error.clone()
    }

    fn extra_info(&self) -> Option<String> {
        self.original_response.clone()
    }
}

impl Github {
    pub fn new(token: String, owner: String) -> Github {
        Github {
            client: reqwest::Client::new(),
            token,
            owner,
        }
    }
    fn add_headers(&self, req: RequestBuilder) -> RequestBuilder {
        req.header(header::AUTHORIZATION, format!("token {}", self.token))
            .header(header::USER_AGENT, "MultiGitRs")
            .header(header::ACCEPT, "application/vnd.github+json")
    }

    async fn send_and_parse(&self, req: RequestBuilder) -> Result<String, reqwest::StatusCode> {
        let r: Response = req.send().await.unwrap();

        match r.error_for_status() {
            Ok(response) => Ok(response.text().await.unwrap()),
            Err(e) => Err(e.status().unwrap()),
        }
    }

    async fn get(
        &self,
        endpoint: String,
        params: Option<&[(&String, &String)]>,
    ) -> Result<String, reqwest::StatusCode> {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.get(url);

        self.send_and_parse(self.add_headers(req).query(&params))
            .await
    }

    async fn post(
        &self,
        endpoint: String,
        params: Option<HashMap<String, &String>>,
    ) -> Result<String, reqwest::StatusCode> {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.post(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }

    async fn put(
        &self,
        endpoint: String,
        params: Option<HashMap<String, &String>>,
    ) -> Result<String, reqwest::StatusCode> {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.put(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }
    async fn delete(
        &self,
        endpoint: String,
        params: Option<HashMap<String, &String>>,
    ) -> Result<String, reqwest::StatusCode> {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.delete(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }
}
