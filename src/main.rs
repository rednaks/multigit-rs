use clap::Parser;
use config::{load_config, Config};
use env_logger::Env;
use github::branches::response::Branch;
use github::commits::response::CompareStatus;
use github::pulls::response::PullRequest;
use github::repos::response::Repo;
use github::Github;
use log::debug;
use log::error;
use log::info;
use log::warn;

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
    create_pulls: bool,

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

fn check_branch_in(branch_name: &String, branches: &Vec<Branch>) -> bool {
    branches
        .iter()
        .map(|b| b.name.clone())
        .any(|b_name| b_name == branch_name.as_str())
}

async fn get_or_create_pull_request(
    gh: &Github,
    repo: &Repo,
    owner: String,
    args: &Aargs,
) -> Option<PullRequest> {
    let pulls: Vec<PullRequest> = match gh.list_pulls(&repo, &args.from, &args.to).await {
        Ok(pulls) => pulls,
        Err(e) => {
            error!(
                "Unable to get pull requests for repo {:?}, err: {}",
                &repo.name,
                e.error_message()
            );
            // TODO: how should this be handled ?
            std::process::exit(-1);
        }
    };

    let existing_pr = pulls.iter().find(|pr| {
        pr.head.label == format!("{}:{}", owner, args.from)
            && pr.base.label == format!("{}:{}", owner, args.to)
    });

    debug!("Matched prs: {:?}", existing_pr);

    match existing_pr {
        Some(pull_request) => {
            let full_pr: Option<PullRequest> = match gh.get_pull(&repo, pull_request.number).await {
                Ok(pr) => {
                    info!("A matching Pull request already exists");
                    Some(pr)
                }
                Err(e) => {
                    error!("Unable to get pull {:?}", e.error_message());
                    None
                }
            };
            full_pr
        }
        None => {
            if args.create_pulls {
                return match gh
                    .create_pull(&repo, &args.from, &args.to, &args.reference)
                    .await
                {
                    Ok(new_pull_request) => Some(new_pull_request),
                    Err(e) => {
                        error!("Unable to create a new PR {}", e.error_message());
                        None
                    }
                };
            }
            None
        }
    }
}

async fn merge_and_delete(gh: &Github, pr: &PullRequest, args: &Aargs) {
    let repo: &Repo = pr.base.repo.as_ref().unwrap();

    info!(
        "Merging {} into {} for {}",
        pr.head.label, pr.base.label, repo.name
    );

    debug!("is mergeable ? {:?}", pr.mergeable.unwrap_or(false));

    match pr.mergeable {
        Some(mergeable) => {
            if mergeable {
                let merge_status = gh.merge_pull(repo, pr).await;
                match merge_status {
                    Ok(merge_status) => {
                        if merge_status.merged == true && args.delete_branches {
                            if let Err(e) = gh.delete_reference(repo, &args.from).await {
                                error!(
                                    "Failed to delete branch {}. reason: {}",
                                    args.from,
                                    e.error_message()
                                );
                                if let Some(extra_info) = e.extra_info() {
                                    debug!("original response: {:?}", extra_info)
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to merge #{}, {}", pr.number, e.error_message());
                        if let Some(extra_info) = e.extra_info() {
                            debug!("original response: {:?}", extra_info);
                        }
                    }
                }
            } else {
                warn!("Pull request is not mergeable, there is a conflict");
            }
        }
        None => {
            info!("Unable to know if it's mergeable, please try later");
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Aargs::parse();
    let config: Config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            error!("{}", e);
            std::process::exit(-1);
        }
    };

    info!("Managing {}", config.org_name);

    let gh = Github::new(config.token, config.org_name.clone());

    for repo_name in config.repos {
        let repo = match gh.get_repo(&repo_name).await {
            Ok(repo) => repo,
            Err(e) => {
                warn!("Unable to get repo {repo_name}: {:?}", e.error_message());
                if let Some(extra_info) = e.extra_info() {
                    debug!("{extra_info}");
                }
                continue;
            }
        };

        info!("Processing repo: {}", repo.name);

        let branches: Vec<Branch> = match gh.list_branches(&repo).await {
            Ok(branches) => branches,
            Err(e) => {
                error!(
                    "Couldn't get branches for repo {:?}, error: {}. Skipping ...",
                    &repo.name,
                    e.error_message()
                );

                if let Some(extra_info) = e.extra_info() {
                    debug!("{}", extra_info);
                }

                // TODO: should we skip or abort ?
                continue;
            }
        };

        for branch in branches.iter() {
            debug!("{}, branch: {}", repo.name, branch.name);
        }

        if !check_branch_in(&args.from, &branches) {
            error!(
                "Source Branch {} doesn't exist for repo {}",
                args.from, repo.name
            );
        }

        if args.create_branches {
            //
            if check_branch_in(&args.to, &branches) {
                if !args.create_pulls {
                    info!(
                        r#"Destination Branch `{}` already exists for repo `{}`.
                Use --create-pulls to create pull requests and update it."#,
                        args.to, repo.name
                    );
                }
            } else {
                let from_ref = match gh.get_reference(&repo, &args.from).await {
                    Ok(from_ref) => from_ref,
                    Err(e) => {
                        error!(
                            "Unable to get reference {}: {:?}",
                            args.from,
                            e.error_message()
                        );
                        continue;
                    }
                };

                match gh.create_reference(&repo, &args.to, &from_ref).await {
                    Ok(_) => {
                        info!("Branch `{}` created successfully on {}", args.to, repo.name);
                        // branch newly created, no need to create a pull request
                        continue;
                    }
                    Err(e) => {
                        error!(
                            "Error on creating branch `{}` for `{}`: {}",
                            args.to,
                            repo.name,
                            e.error_message()
                        );
                    }
                };
            }
        }

        if !check_branch_in(&args.to, &branches) {
            error!(
                r#"Destination Branch `{}` doesn't exist for repo `{}`.
                Use --create-branches to create it or create it manually on gh."#,
                args.to, repo.name
            );
            continue;
        }

        let mut pull_request: Option<PullRequest> = None;

        if args.create_pulls {
            //
            info!("Comparing {} and {} for PR", args.to, args.from);
            let comp = match gh.compare_branches(&repo, &args.to, &args.from).await {
                Ok(comp) => comp,
                Err(e) => {
                    error!(
                        "Unable to get comparison between {} and {} : {}",
                        args.to,
                        args.from,
                        e.error_message()
                    );
                    if let Some(extra_info) = e.extra_info() {
                        debug!("{}", extra_info);
                    }
                    continue;
                }
            };

            info!("`{}` is {:?} to `{}`", args.to, comp.status, args.from);
            pull_request = match comp.status {
                CompareStatus::Behind | CompareStatus::Diverged => {
                    info!(
                        "Creating pull request from {} into {} for {}",
                        args.from, args.to, repo.name
                    );
                    get_or_create_pull_request(&gh, &repo, config.org_name.clone(), &args).await
                }
                _ => {
                    info!("Nothing to merge !");
                    None
                }
            };
        }

        if args.merge {
            if let Some(pr) = pull_request {
                // merge
                info!("Merging {} for {}", pr.title, repo.name);
                merge_and_delete(&gh, &pr, &args).await;
            } else {
                info!("No pull requests to merge for {}", repo.name);
            }
        }
    }
}
