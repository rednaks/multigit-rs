use clap::Parser;
use config::{load_config, Config};
use env_logger::Env;
use exitcode;
use github::branches::response::Branch;
use github::commits::response::CompareStatus;
use github::pulls::response::PullRequest;
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

fn check_branch_in(branch_name: &String, branches: &Vec<Branch>) -> bool {
    branches
        .iter()
        .map(|b| b.name.clone())
        .any(|b_name| b_name == branch_name.as_str())
}

async fn get_or_create_pull_request(
    gh: &Github,
    repo_name: &String,
    owner: String,
    args: &Aargs,
) -> Option<PullRequest> {
    let pulls: Vec<PullRequest> = match gh.list_pulls(&repo_name, &args.from, &args.to).await {
        Ok(pulls) => pulls,
        Err(e) => {
            error!(
                "Unable to get pull requests for repo {:?}, err: {}",
                &repo_name,
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
            let full_pr: Option<PullRequest> =
                match gh.get_pull(&repo_name, pull_request.number).await {
                    Ok(pr) => Some(pr),
                    Err(e) => {
                        error!("Unable to get pull {:?}", e.error_message());

                        None
                    }
                };
            full_pr
        }
        None => {
            if args.create {
                return match gh
                    .create_pull(&repo_name, &args.from, &args.to, &args.reference)
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
    let repo_name: String = pr.base.repo.as_ref().unwrap().name.clone();

    info!(
        "Merging {} into {} for {}",
        pr.head.label, pr.base.label, repo_name
    );

    debug!("is mergeable ? {:?}", pr.mergeable.unwrap_or(false));

    match pr.mergeable {
        Some(mergeable) => {
            if mergeable {
                let merge_status = gh.merge_pull(&repo_name, pr).await;
                match merge_status {
                    Ok(merge_status) => {
                        if merge_status.merged == true && args.delete_branches {
                            match gh.delete_reference(&repo_name, &args.from).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!(
                                        "Failed to delete branch {}. reason: {}",
                                        args.from,
                                        e.error_message()
                                    );
                                    match e.extra_info() {
                                        Some(extra_info) => {
                                            debug!("original response: {:?}", extra_info)
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to merge #{}, {}", pr.number, e.error_message());
                        match e.extra_info() {
                            Some(extra_info) => debug!("original response: {:?}", extra_info),
                            None => {}
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
        let branches: Vec<Branch> = match gh.list_branches(&repo_name).await {
            Ok(branches) => branches,
            Err(e) => {
                error!(
                    "Couldn't get branches for repo {:?}, error: {}. Skipping ...",
                    &repo_name,
                    e.error_message()
                );

                match e.extra_info() {
                    Some(extra_info) => debug!("{}", extra_info),
                    _ => {
                        //
                    }
                }
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

        let comp = match gh.compare_branches(&repo_name, &args.to, &args.from).await {
            Ok(comp) => comp,
            Err(e) => {
                error!(
                    "Unable to get comparison between {} and {} : {}",
                    args.to,
                    args.from,
                    e.error_message()
                );
                match e.extra_info() {
                    Some(extra_info) => debug!("{}", extra_info),
                    None => {}
                };
                std::process::exit(-1);
            }
        };

        let pull_request: Option<PullRequest> = match comp.status {
            CompareStatus::Behind | CompareStatus::Diverged => {
                get_or_create_pull_request(&gh, &repo_name, config.org_name.clone(), &args).await
            }
            _ => {
                info!("Nothing to merge !");
                None
            }
        };

        debug!("PR: {:?}", pull_request.clone().unwrap().title);

        match pull_request {
            Some(pr) => {
                if !args.merge {
                    info!("Merge not requested, nothing to do !");
                    std::process::exit(0);
                }
                merge_and_delete(&gh, &pr, &args).await;
            }
            None => {
                //
            }
        }
    }
}
