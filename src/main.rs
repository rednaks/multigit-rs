use clap::Parser;
use config::{load_config, Config};
use exitcode;
use github::{CompareStatus, Github, GithubBranch, GithubPullRequest, MergeStatus};
use log::debug;
use log::error;
use log::info;

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

fn check_branch_in(branch_name: &String, branches: &Vec<GithubBranch>) -> bool {
    branches
        .iter()
        .map(|b| b.name.clone())
        .any(|b_name| b_name == branch_name.as_str())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Aargs::parse();
    let config: Config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            error!("{}", e);
            std::process::exit(-1);
        }
    };

    info!("Managing {}", config.org_name);

    let gh = Github::new(config.token, config.org_name);

    for repo_name in config.repos {
        let branches: Vec<GithubBranch> = match gh.list_branches(&repo_name).await {
            Some(branches) => branches,
            None => {
                error!(
                    "Couldn't get branches for repo {:?}. Skipping ...",
                    &repo_name
                );
                // TODO: should we skip or abort ?
                continue;
            }
        };

        for branch in branches.iter() {
            debug!("branch: {}", branch.name);
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

        let comp = gh.compare_branches(&repo_name, &args.to, &args.from).await;
        debug!(
            "{} comp {} and {}: {:?}",
            repo_name, args.to, args.from, comp
        );

        let pull_request: Option<GithubPullRequest> = match comp {
            CompareStatus::Behind | CompareStatus::Diverged => {
                let pulls: Vec<GithubPullRequest> =
                    match gh.list_pulls(&repo_name, &args.from, &args.to).await {
                        Some(pulls) => pulls,
                        None => {
                            error!("Unable to get pull requests for repo {:?}", &repo_name);
                            // TODO: how should this be handled ?
                            std::process::exit(-1);
                        }
                    };

                if pulls.len() > 0 {
                    // TODO: is there a better way ?
                    info!("PRs already exists ?: {:?}", pulls);
                    Some(pulls[0].clone())
                } else if args.create {
                    info!("Creating pull request for repo {}", repo_name);
                    gh.create_pull(&repo_name, &args.from, &args.to, &args.reference)
                        .await
                } else {
                    None
                }
            }
            _ => {
                info!("Nothing to merge !");
                None
            }
        };

        debug!("PR: {:?}", pull_request);

        match pull_request {
            Some(pr) => {
                if !args.merge {
                    info!("Merge not requested, nothing to do !");
                    std::process::exit(0);
                }
                let merge_status = gh.merge_pull(&repo_name, &pr.number).await;
                match merge_status {
                    MergeStatus::Success => {
                        if args.delete_branches {
                            gh.delete_branches(&repo_name, &args.from).await;
                        }
                    }
                    MergeStatus::Failed => {
                        error!("Failed to merge #{} ", pr.number)
                    }
                }
            }
            None => {
                //
            }
        }
        if args.merge {}
    }
}
