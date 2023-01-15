use super::response::User;
use crate::Github;
use crate::GithubAPIError;
use crate::GithubAPIResponseDeserializeError;
use crate::GithubAPIResponseError;

impl Github {
    pub async fn get_me(&self) -> Result<User, Box<dyn GithubAPIError>> {
        //
        match self.get(String::from("user"), None).await {
            Ok(response) => {
                let deserializer = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<User, _> = serde_path_to_error::deserialize(deserializer);
                if let Err(e) = result {
                    return Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get authenticated user: {}", e),
                        original_response: Some(response),
                    }));
                }
                Ok(result.unwrap())
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::UNAUTHORIZED => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Authentication required"),
                })),
                reqwest::StatusCode::FORBIDDEN => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Forbidden"),
                })),
                _ => Err(Box::new(GithubAPIResponseError {
                    message: format!("Unhandled: {}", status_code),
                })),
            },
        }
    }
}
