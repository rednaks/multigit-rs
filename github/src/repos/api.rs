use crate::Github;
use crate::GithubAPIError;
use crate::GithubAPIResponseDeserializeError;
use crate::GithubAPIResponseError;

use super::response::Repo;

impl Github {
    pub async fn get_repo(&self, repo: &String) -> Result<Repo, Box<dyn GithubAPIError>> {
        let endpoint: String = format!("repos/{}/{}", self.owner, repo);
        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Repo, _> = serde_path_to_error::deserialize(ds);
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
                    message: format!("Unhandled {}", status_code),
                })),
            },
        }
    }

    pub async fn list_repos(
        &self,
        owner: &String,
        is_user: &Option<bool>,
    ) -> Result<Vec<Repo>, Box<dyn GithubAPIError>> {
        let mut endpoint = format!("orgs/{owner}/repos");
        if is_user.unwrap_or(false) {
            endpoint = format!("users/{owner}/repos");
        }

        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Vec<Repo>, _> = serde_path_to_error::deserialize(ds);
                match result {
                    Ok(repos_list) => Ok(repos_list),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to parse response : {:?}", e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                _ => Err(Box::new(GithubAPIResponseError {
                    message: format!("Unhandled status code: {}", status_code),
                })),
            },
        }
    }
}
