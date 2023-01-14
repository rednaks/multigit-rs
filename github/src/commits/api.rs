use super::response::CommitsComparison;
use crate::repos::response::Repo;
use crate::Github;
use crate::GithubAPIError;
use crate::GithubAPIResponseDeserializeError;
use crate::GithubAPIResponseError;

impl Github {
    pub async fn compare_branches(
        &self,
        repo: &Repo,
        base: &String,
        head: &String,
    ) -> Result<CommitsComparison, Box<dyn GithubAPIError>> {
        let endpoint = format!(
            "repos/{}/{}/compare/{}...{}",
            self.owner, repo.name, head, base
        );

        match self.get(endpoint, None).await {
            Ok(response) => {
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<CommitsComparison, _> = serde_path_to_error::deserialize(ds);

                match result {
                    Ok(comparison) => Ok(comparison),
                    Err(e) => Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get comparison: {:?}", e),
                        original_response: Some(response),
                    })),
                }
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::NOT_FOUND => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Not found"),
                })),
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Internal Error"),
                    }))
                } // handle _ case
                _ => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Unhandled"),
                })),
            },
        }
    }
}
