mod github;
use github::{CompareStatus, Github, MergeStatus};
use log::debug;
use log::error;
use log::info;

use clap::Parser;
use exitcode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Parser, Debug)]
#[clap(author, version, long_about=None)]
struct Aargs {
    #[clap(long, value_parser)]
    /// source branch to create pull request
    from: String,

    #[clap(long, value_parser)]
    /// destination branch to create pull request
    to: String,

    #[clap(long, value_parser)]
    /// reference branch: org/project#issue_number
    reference: String,

    #[clap(long, value_parser)]
    /// create pull requests if not existing
    create: bool,
    #[clap(long, value_parser)]
    /// merge pull requests
    merge: bool,
    #[clap(long, value_parser)]
    /// merge pull requests
    list: bool,
    #[clap(long, value_parser)]
    /// create branches if they don't exist
    create_branches: bool,
    #[clap(long, value_parser)]
    /// delete branches after merge
    delete_branches: bool,
}

#[derive(Deserialize, Serialize)]
struct Config {
    token: String,
    org_name: String,
    is_user: bool,
    repos: Vec<String>,
}

fn check_branch_in(branch_name: &String, branches: &Vec<Value>) -> bool {
    branches
        .iter()
        .map(|b| b["name"].as_str())
        .any(|b_name| b_name.unwrap() == branch_name.as_str())
}

fn load_config() -> Result<Config, ()> {
    let text = std::fs::read_to_string("config.json").expect("Err");
    match serde_json::from_str(&text) {
        Ok(config) => Ok(config),
        Err(e) => {
            error!("Unable to load config: {:?}", e);
            Err(())
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Aargs::parse();
    let config: Config = match load_config() {
        Ok(config) => config,
        Err(_) => {
            std::process::exit(-1);
        }
    };

    info!("Managing {}", config.org_name);

    let gh = Github {
        client: reqwest::Client::new(),
        token: config.token,
        owner: config.org_name,
    };

    for repo_name in config.repos {
        let branches: Vec<Value> = gh.list_branches(&repo_name).await;

        for branch in branches.iter() {
            debug!("branch: {}", branch["name"]);
        }

        if !check_branch_in(&args.from, &branches) {
            error!(
                "Source Branch {} doesn't exist for repo {}",
                args.from, repo_name
            );
        }

        if !check_branch_in(&args.to, &branches) {
            if args.create_branches {
                // TODO
                error!(
                    "Destination Branch `{}` doesn't exist for repo `{}`.",
                    args.to, repo_name
                );
            } else {
                error!(
                    r#"Destination Branch `{}` doesn't exist for repo `{}`.
                Use --create-branches to create it or create it manually on gh."#,
                    args.to, repo_name
                );

                std::process::exit(exitcode::CONFIG);
            }
        }

        let comp = gh.compare(&repo_name, &args.to, &args.from).await;
        debug!(
            "{} comp {} and {}: {:?}",
            repo_name, args.to, args.from, comp
        );

        let pull_request: Value = match comp {
            CompareStatus::Behind | CompareStatus::Diverged => {
                let pulls: Vec<Value> = gh.list_pulls(&repo_name, &args.from, &args.to).await;

                if pulls.len() > 0 {
                    // TODO: is there a better way ?
                    info!("PRs already exists ?: {:?}", pulls);
                    serde_json::from_str::<Value>(pulls[0].to_string().as_str()).unwrap()
                } else if args.create {
                    info!("Creating pull request for repo {}", repo_name);
                    gh.create_pull(&repo_name, &args.from, &args.to, &args.reference)
                        .await
                } else {
                    serde_json::json!(null)
                }
            }
            _ => {
                info!("Nothing to merge !");
                serde_json::json!(null)
            }
        };

        debug!("PR: {:?}", pull_request);

        if args.merge {
            let status = gh
                .merge_pull(&repo_name, &pull_request["number"].as_u64().unwrap())
                .await;

            match status {
                MergeStatus::Success => {
                    if args.delete_branches {
                        gh.delete_branches(&repo_name, &args.from).await;
                    }
                }
                MergeStatus::Failed => {
                    error!(
                        "Failed to merge #{} ",
                        pull_request["number"].as_u64().unwrap()
                    )
                }
            }
        }
    }
}
