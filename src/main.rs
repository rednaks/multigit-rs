mod github;
use github::{CompareStatus, Github, MergeStatus};

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

fn load_config() -> Config {
    let text = std::fs::read_to_string("config.json").expect("Err");
    serde_json::from_str(&text).unwrap()
}

#[tokio::main]
async fn main() {
    let args = Aargs::parse();
    let config: Config = load_config();

    println!("Managing {}", config.org_name);

    let gh = Github {
        client: reqwest::Client::new(),
        token: config.token,
        owner: config.org_name,
    };

    for repo_name in config.repos {
        let branches: Vec<Value> = gh.list_branches(repo_name.as_str()).await;

        for branch in branches.iter() {
            println!("branch: {}", branch["name"]);
        }

        if !check_branch_in(&args.from, &branches) {
            panic!(
                "Source Branch {} doesn't exist for repo {}",
                args.from, repo_name
            );
        }

        if !check_branch_in(&args.to, &branches) {
            if args.create_branches {
                // TODO
                panic!(
                    "Destination Branch {} doesn't exist for repo {}.",
                    args.from, repo_name
                );
            } else {
                panic!(
                    r#"Destination Branch {} doesn't exist for repo {}.
                Use --create-branches to create it or create it manually on gh."#,
                    args.from, repo_name
                );

                std::process::exit(exitcode::CONFIG);
            }
        }

        let comp = gh.compare(&repo_name, &args.to, &args.from).await;

        let pull_request: Value = match comp {
            CompareStatus::Behind | CompareStatus::Diverged => {
                let pulls: Vec<Value> = gh
                    .list_pulls(repo_name.as_str(), &args.from, &args.to)
                    .await;

                if pulls.len() > 0 {
                    // TODO: is there a better way ?
                    serde_json::from_str::<Value>(pulls[0].to_string().as_str()).unwrap()
                } else if args.create {
                    gh.create_pull(repo_name.as_str(), &args.from, &args.to)
                        .await
                } else {
                    serde_json::json!(null)
                }
            }
            _ => {
                println!("Nothing to merge !");
                serde_json::json!(null)
            }
        };

        if args.merge {
            let status = gh
                .merge_pull(
                    repo_name.as_str(),
                    &pull_request["number"].as_u64().unwrap(),
                )
                .await;

            match status {
                MergeStatus::Success => {
                    if args.delete_branches {
                        gh.delete_branches(repo_name.as_str(), &args.from).await;
                    }
                }
                MergeStatus::Failed => {
                    (println!(
                        "Failed to merge #{} ",
                        pull_request["number"].as_u64().unwrap()
                    ))
                }
            }
        }
    }
}
