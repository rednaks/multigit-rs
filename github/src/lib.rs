pub mod commits;
mod github;
pub mod pulls;
mod response;

pub use response::{GithubBranch, GithubDeleteReference, GithubRepo};

pub use pulls::response::{
    GithubPullRequest, GithubPullRequestMergeStatus, GithubPullRequestState,
};

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

    // pub async fn list_repos(&self, is_user: &Option<bool>) -> Vec<GithubRepo> {
    //     let mut endpoint = format!("orgs/{}/repos", self.owner);
    //     if is_user.unwrap_or(false) {
    //         endpoint = format!("users/{}/repos", self.owner);
    //     }

    //     match self.get(endpoint, None).await {
    //         Ok(response) => serde_json::from_str::<Vec<GithubRepo>>(&response).unwrap(),
    //         Err(_) => {
    //             match status_code {
    //                 reqwest::StatusCode
    //             }
    //         }
    //     }
    // }

    pub async fn get_repo(&self, repo: &String) -> Result<GithubRepo, Box<dyn GithubAPIError>> {
        let endpoint: String = format!("repos/{}/{}", self.owner, repo);
        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<GithubRepo, _> = serde_path_to_error::deserialize(ds);
                match result {
                    Ok(repo) => Ok(repo),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get repo : {}: {}", repo, e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::FORBIDDEN => Err(Box::new(GithubAPIResponseError {
                    message: String::from("You are not authorized to get this repo"),
                })),
                reqwest::StatusCode::NOT_FOUND => Err(Box::new(GithubAPIResponseError {
                    message: String::from("You are not authorized to get this repo"),
                })),
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }

    pub async fn list_branches(
        &self,
        repo: &String,
    ) -> Result<Vec<GithubBranch>, Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/branches", self.owner, repo);

        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Vec<GithubBranch>, _> = serde_path_to_error::deserialize(ds);
                match result {
                    Ok(branches) => Ok(branches),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Error: Unable to get branches: {:?}", e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::NOT_FOUND => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Resource Not found"),
                })),
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }

    pub async fn delete_reference(
        &self,
        repo: &String,
        reference: &String,
    ) -> Result<(), Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/git/refs/{}", self.owner, repo, reference);

        match self.delete(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<GithubDeleteReference, _> = serde_path_to_error::deserialize(ds);
                match result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: String::from(""),
                        original_response: None,
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Validation error"),
                    }))
                }
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }
}
