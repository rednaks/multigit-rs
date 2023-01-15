use super::response::Org;
use crate::Github;
use crate::GithubAPIError;
use crate::GithubAPIResponseDeserializeError;
use crate::GithubAPIResponseError;

impl Github {
    pub async fn get_org(&self, org_name: &String) -> Result<Org, Box<dyn GithubAPIError>> {
        match self.get(format!("orgs/{org_name}"), None).await {
            Ok(response) => {
                //
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Org, _> = serde_path_to_error::deserialize(ds);
                if let Err(e) = result {
                    return Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get orgs response: {:?}", e),
                        original_response: Some(response),
                    }));
                }
                Ok(result.unwrap())
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::NOT_FOUND => Err(Box::new(GithubAPIResponseError {
                    message: String::from("Not found"),
                })),
                _ => Err(Box::new(GithubAPIResponseError {
                    message: format!("Unhandled error {}", status_code),
                })),
            },
        }
    }

    pub async fn get_my_orgs(&self) -> Result<Vec<Org>, Box<dyn GithubAPIError>> {
        match self.get(String::from("user/orgs"), None).await {
            Ok(response) => {
                //
                let ds = &mut serde_json::Deserializer::from_str(&response);
                let result: Result<Vec<Org>, _> = serde_path_to_error::deserialize(ds);
                if let Err(e) = result {
                    return Err(Box::new(GithubAPIResponseDeserializeError {
                        parse_error: format!("Unable to get orgs response: {:?}", e),
                        original_response: Some(response),
                    }));
                }
                Ok(result.unwrap())
            }
            Err(status_code) => match status_code {
                reqwest::StatusCode::FORBIDDEN | reqwest::StatusCode::UNAUTHORIZED => {
                    Err(Box::new(GithubAPIResponseError {
                        message: String::from("Not authorized"),
                    }))
                }
                _ => Err(Box::new(GithubAPIResponseError {
                    message: format!("Unhandled error {}", status_code),
                })),
            },
        }
    }
}
