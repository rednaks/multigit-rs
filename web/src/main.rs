use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use config::load_config;
use github::orgs::response::Org;
use github::repos::response::Repo;
use github::users::response::User;
use github::Github;
use log::error;
use web_common::{OrgResponse, OrgType};

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

#[get("/api/orgs/{org}")]
async fn manage_org(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let org_name: String = path.into_inner();

    println!("Managing: {}", org_name);
    let the_org: Org = match data.gh.get_org(&org_name).await {
        Ok(a_org) => a_org,
        Err(e) => {
            println!("Unable to get the org: {org_name}: {:?}", e.error_message());
            ::std::process::exit(-1);
        }
    };

    let repos: Vec<Repo> = match data.gh.list_repos(&the_org.login, &Some(false)).await {
        Ok(r) => r,
        Err(e) => {
            println!("Couldn't get repos: {}", e.error_message());
            ::std::process::exit(-1);
        }
    };

    HttpResponse::Ok().json(repos)
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
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
