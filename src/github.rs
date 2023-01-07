use std::collections::HashMap;

use log::debug;
use log::error;
use reqwest::{header, RequestBuilder};
use serde_json::Value;

pub struct Github {
    pub client: reqwest::Client,
    pub owner: String,
    pub token: String,
}

#[derive(Debug)]
pub enum CompareStatus {
    Ahead,
    Behind,
    Diverged,
    Identical,
}

pub enum MergeStatus {
    Success,
    Failed,
}

impl Github {
    fn add_headers(&self, req: RequestBuilder) -> RequestBuilder {
        req.header(header::AUTHORIZATION, format!("token {}", self.token))
            .header(header::USER_AGENT, "MultiGitRs")
            .header(header::ACCEPT, "application/vnd.github+json")
    }

    async fn send_and_parse(&self, req: RequestBuilder) -> String {
        req.send().await.unwrap().text().await.unwrap()
    }

    async fn get(&self, endpoint: String, params: Option<&[(&String, &String)]>) -> String {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.get(url);

        self.send_and_parse(self.add_headers(req).query(&params))
            .await
    }

    async fn post(&self, endpoint: String, params: Option<HashMap<String, &String>>) -> String {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.post(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }

    async fn put(&self, endpoint: String, params: Option<HashMap<String, &String>>) -> String {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.put(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }

    async fn delete(&self, endpoint: String, params: Option<HashMap<String, &String>>) -> String {
        let url = format!("https://api.github.com/{}", endpoint);

        let req = self.client.delete(url);
        self.send_and_parse(self.add_headers(req).json(&params))
            .await
    }

    pub async fn list_repos(&self, is_user: &Option<bool>) -> Vec<Value> {
        let mut endpoint = format!("orgs/{}/repos", self.owner);
        if is_user.unwrap_or(false) {
            endpoint = format!("users/{}/repos", self.owner);
        }

        let response = self.get(endpoint, None).await;

        serde_json::from_str::<Vec<Value>>(&response).unwrap()
    }

    pub async fn get_repo(&self, repo: &String) -> Value {
        let endpoint: String = format!("repos/{}/{}", self.owner, repo);
        let response = self.get(endpoint, None).await;

        serde_json::from_str::<Value>(&response).unwrap()
    }

    pub async fn list_branches(&self, repo: &String) -> Vec<Value> {
        let endpoint = format!("repos/{}/{}/branches", self.owner, repo);

        let response = self.get(endpoint, None).await;

        serde_json::from_str::<Vec<Value>>(&response).unwrap()
    }

    pub async fn compare(&self, repo: &String, base: &String, head: &String) -> CompareStatus {
        let endpoint = format!("repos/{}/{}/compare/{}...{}", self.owner, repo, base, head);

        let response = self.get(endpoint, None).await;

        let parsed = serde_json::from_str::<Value>(&response).unwrap();

        // compare don't show when refs are conflicting.

        match parsed["status"].as_str() {
            Some(a_status) => match a_status {
                "ahead" => CompareStatus::Ahead,
                "behind" => CompareStatus::Behind,
                "diverged" => CompareStatus::Diverged,
                "identical" => CompareStatus::Identical,
                _ => {
                    panic!("Comparision not handleld !")
                }
            },
            None => {
                panic!("No status returned !");
            }
        }
    }

    pub async fn list_pulls(&self, repo: &String, from: &String, to: &String) -> Vec<Value> {
        let endpoint = format!("repos/{}/{}/pulls", self.owner, repo);
        let response = self
            .get(
                endpoint,
                Some(&[
                    (&String::from("state"), &String::from("open")),
                    (&String::from("head"), from),
                    (&String::from("base"), to),
                ]),
            )
            .await;

        serde_json::from_str::<Vec<Value>>(&response).unwrap()
    }

    pub async fn create_pull(
        &self,
        repo: &String,
        from: &String,
        to: &String,
        reference: &String,
    ) -> Value {
        let endpoint = format!("repos/{}/{}/pulls", self.owner, repo);
        let mut params = HashMap::<String, &String>::with_capacity(2);
        let title: String = format!("PR for: {}. {} into {}", reference, from, to);
        params.insert(String::from("title"), &title);
        params.insert(String::from("base"), to);
        params.insert(String::from("head"), from);

        let response = self.post(endpoint, Some(params)).await;

        serde_json::from_str::<Value>(&response).unwrap()
    }

    pub async fn merge_pull(&self, repo: &String, pull_number: &u64) -> MergeStatus {
        let endpoint = format!("repos/{}/{}/pulls/{}/merge", self.owner, repo, pull_number);

        let response = self.put(endpoint, None).await;

        let parsed = serde_json::from_str::<Value>(&response).unwrap();

        match parsed["merged"].as_bool().unwrap_or(false) {
            true => MergeStatus::Success,
            false => MergeStatus::Failed,
        }
    }

    pub async fn delete_branches(&self, repo: &String, branch: &String) {
        let endpoint = format!("repos/{}/{}/git/refs/{}", self.owner, repo, branch);

        let res = self.delete(endpoint, None).await;
        debug!("{}", res);
    }
}
