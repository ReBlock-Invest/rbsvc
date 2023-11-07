use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

mod login;
mod uid;

async fn halo() -> impl Responder {
    format!("Halo!")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(halo))
            .route("/health", web::get().to(health_check))
            .route("/issue", web::post().to(uid::issue_uid))
            .route("/nonce", web::post().to(login::nonce))
            .route("/wallet/login", web::post().to(login::wallet_login))
    })
    .bind("127.0.0.1:8000")?
    .run();

    Ok(server)
}
