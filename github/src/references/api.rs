use std::collections::HashMap;

use super::response::{DeleteReference, Reference};
use crate::repos::response::Repo;
use crate::Github;
use crate::{GithubAPIError, GithubAPIResponseDeserializeError, GithubAPIResponseError};

impl Github {
    pub async fn get_reference(
        &self,
        repo: &Repo,
        reference: &String,
    ) -> Result<Reference, Box<dyn GithubAPIError>> {
        let endpoint = format!(
            "repos/{}/{}/git/refs/heads/{reference}",
            self.owner, repo.name
        );

        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Reference, _> = serde_path_to_error::deserialize(ds);
                match result {
                    Ok(a_reference) => Ok(a_reference),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to parse ref get: {:?}", e),
                        original_response: Some(response),
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
                    message: format!("Unhandled: {}", status_code),
                })),
            },
        }
    }

    pub async fn create_reference(
        &self,
        repo: &Repo,
        branch_name: &String,
        from_ref: &Reference,
    ) -> Result<(), Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/git/refs", self.owner, repo.name);

        let mut params = HashMap::<String, &String>::with_capacity(2);
        let ref_ = format!("refs/head/{branch_name}");
        params.insert(String::from("ref"), &ref_);
        params.insert(String::from("sha"), &from_ref.object.sha);

        match self.post(endpoint, Some(params)).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<DeleteReference, _> = serde_path_to_error::deserialize(ds);
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
                    message: format!("Unhandled: {}", status_code),
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
                let result: Result<DeleteReference, _> = serde_path_to_error::deserialize(ds);
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
                    message: format!("Unhandled: {}", status_code),
                })),
            },
        }
    }
}
