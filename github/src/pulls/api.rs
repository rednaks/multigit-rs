use super::response::{GithubPullRequest, GithubPullRequestMergeStatus};
use crate::Github;
use crate::GithubAPIError;
use crate::GithubAPIResponseDeserializeError;
use crate::GithubAPIResponseError;
use std::collections::HashMap;

impl Github {
    pub async fn list_pulls(
        &self,
        repo: &String,
        from: &String,
        to: &String,
    ) -> Result<Vec<GithubPullRequest>, Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/pulls", self.owner, repo);
        match self
            .get(
                endpoint,
                Some(&[
                    (&String::from("state"), &String::from("open")),
                    (&String::from("head"), from),
                    (&String::from("base"), to),
                ]),
            )
            .await
        {
            Ok(response) => {
                let deserializer = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Vec<GithubPullRequest>, _> =
                    serde_path_to_error::deserialize(deserializer);

                match result {
                    Ok(pull_requests) => Ok(pull_requests),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get list of pull requests: {}", e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Invalid"),
                    }))
                }
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }

    pub async fn create_pull(
        &self,
        repo: &String,
        from: &String,
        to: &String,
        reference: &String,
    ) -> Result<GithubPullRequest, Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/pulls", self.owner, repo);
        let mut params = HashMap::<String, &String>::with_capacity(2);
        let title: String = format!("PR for: {}. {} into {}", reference, from, to);
        params.insert(String::from("title"), &title);
        params.insert(String::from("base"), to);
        params.insert(String::from("head"), from);

        match self.post(endpoint, Some(params)).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<GithubPullRequest, _> = serde_path_to_error::deserialize(ds);

                match result {
                    Ok(pr) => Ok(pr),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to create pull request: {}", e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::FORBIDDEN => Err(Box::new(GithubAPIResponseError {
                    message: String::from("You are not allowed to create a pull request"),
                })),
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Invalid request"),
                    }))
                }
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }

    pub async fn merge_pull(
        &self,
        repo: &String,
        pull_number: &u64,
    ) -> Result<GithubPullRequestMergeStatus, Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/pulls/{}/merge", self.owner, repo, pull_number);

        match self.put(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<GithubPullRequestMergeStatus, _> =
                    serde_path_to_error::deserialize(ds);

                match result {
                    Ok(merge_status) => Ok(merge_status),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!(
                            "Error while merging pull request {}: {}",
                            pull_number, e
                        ),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Internal Error"),
                    }))
                }
                reqwest::StatusCode::SERVICE_UNAVAILABLE => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Service unavailable"),
                })),
                reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Unprocessable entity"),
                    }))
                }
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }
}
