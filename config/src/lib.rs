use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub token: String,
    pub org_name: String,
    pub is_user: bool,
    pub repos: Vec<String>,
}

pub fn load_config() -> Result<Config, String> {
    let text = std::fs::read_to_string("config.json").expect("Err");
    let ds = &mut serde_json::Deserializer::from_str(&text);
    let result: Result<Config, _> = serde_path_to_error::deserialize(ds);
    match result {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Unable to load config: {}", e)),
    }
}
