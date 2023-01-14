use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
