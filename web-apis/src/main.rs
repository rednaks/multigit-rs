use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use config::load_config;
use github::orgs::response::Org;
use github::repos::response::Repo;
use github::users::response::User;
use github::Github;
use log::error;
use serde_derive::Deserialize;
use web_common::{OrgResponse, OrgType, RepoResponse};

struct AppState {
    gh: Github,
}

#[get("/api/orgs")]
async fn root(data: web::Data<AppState>) -> impl Responder {
    let orgs: Vec<Org> = match data.gh.get_my_orgs().await {
        Ok(orgs) => orgs,
        Err(e) => {
            println!("Unable to get user's orgs: {}", e.error_message());
            vec![]
        }
    };

    let me: Option<User> = match data.gh.get_me().await {
        Ok(user) => Some(user),
        Err(_) => None,
    };

    let mut orgs_response: Vec<OrgResponse> = orgs
        .iter()
        .map(|o| OrgResponse {
            login: o.login.clone(),
            org_type: OrgType::Organization,
        })
        .collect();

    if let Some(user) = me {
        orgs_response.insert(
            0,
            OrgResponse {
                login: user.login.clone(),
                org_type: OrgType::User,
            },
        );
    }

    HttpResponse::Ok().json(orgs_response)
}

#[derive(Deserialize)]
pub struct OrgInfo {
    #[serde(rename = "type")]
    pub org_type: OrgType,
}

#[get("/api/orgs/{org}/repos")]
async fn manage_org(
    data: web::Data<AppState>,
    path: web::Path<String>,
    org_info: web::Query<OrgInfo>,
) -> impl Responder {
    let org_name: String = path.into_inner();

    println!("Managing: {}", org_name);

    let is_user = org_info.org_type == OrgType::User;

    let repos: Vec<Repo> = match data.gh.list_repos(&org_name, &Some(is_user)).await {
        Ok(r) => r,
        Err(e) => {
            println!("Couldn't get repos: {}", e.error_message());
            ::std::process::exit(-1);
        }
    };
    let repo_response: Vec<RepoResponse> = repos
        .iter()
        .map(|r| RepoResponse {
            name: r.name.clone(),
        })
        .collect();

    HttpResponse::Ok().json(repo_response)
}

#[get("/api/orgs/{org}")]
async fn get_org(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let org_name: String = path.into_inner();

    println!("Getting: {}", org_name);

    let org_response: OrgResponse = match data.gh.get_org(&org_name).await {
        Ok(org) => OrgResponse {
            login: org.login,
            org_type: OrgType::Organization,
        },
        Err(e) => {
            println!("Maybe not an org ? : {}", e.error_message());
            // try user:
            // todo: handle not me.
            match data.gh.get_me().await {
                Ok(user) => OrgResponse {
                    login: user.login,
                    org_type: OrgType::User,
                },
                Err(_) => {
                    println!("{org_name} Not found");
                    return HttpResponse::NotFound().body("Not found");
                }
            }
        }
    };

    HttpResponse::Ok().json(org_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            error!("{}", e);
            std::process::exit(-1);
        }
    };

    let cfg = web::Data::new(AppState {
        gh: Github::new(config.token, config.org_name),
    });

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(cfg.clone())
            .wrap(cors)
            .service(root)
            .service(manage_org)
            .service(get_org)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
