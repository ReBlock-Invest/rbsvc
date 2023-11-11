use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};

mod login;
mod uid;
mod user;
mod wh;

async fn halo() -> impl Responder {
    format!("Halo!")
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/", web::get().to(halo))
            .route("/health", web::get().to(health_check))
            .route("/issue", web::post().to(uid::issue_uid))
            .route("/nonce", web::post().to(login::nonce))
            .route("/wallet/login", web::post().to(login::wallet_login))
            .route("/userinfo", web::get().to(user::user_info))
            .route("/wh-xxx", web::post().to(wh::webhook))
    })
    .bind("0.0.0.0:8000")?
    .run();

    Ok(server)
}
