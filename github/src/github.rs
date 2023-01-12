pub struct Github {
    pub client: reqwest::Client,
    pub owner: String,
    pub token: String,
}
