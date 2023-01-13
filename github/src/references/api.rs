use super::response::DeleteReference;
use crate::Github;
use crate::{GithubAPIError, GithubAPIResponseDeserializeError, GithubAPIResponseError};

impl Github {
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
