use crate::Github;
use crate::{GithubAPIError, GithubAPIResponseDeserializeError, GithubAPIResponseError};

use super::response::Branch;

impl Github {
    pub async fn list_branches(
        &self,
        repo: &String,
    ) -> Result<Vec<Branch>, Box<dyn GithubAPIError>> {
        let endpoint = format!("repos/{}/{}/branches", self.owner, repo);

        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Vec<Branch>, _> = serde_path_to_error::deserialize(ds);
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
}
