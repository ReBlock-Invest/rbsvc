use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use eyre::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct UserResp {
    trx_id: String,
}

fn connect() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("missing environment variable REDIS_URL");

    redis::Client::open(redis_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub async fn user_info(req: HttpRequest) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            let mut conn = connect();
            let key = format!("rb:session:{}", token);

            let user_address = conn.get(&key).unwrap_or_else(|_err| {
                return String::from("");
            });
        } else {
            return HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("{\"error\": \"Invalid auth header\"}");
        }
    } else {
        return HttpResponse::Unauthorized()
            .content_type(ContentType::json())
            .body("{\"error\": \"Missing auth header\"}");
    }

    let resp = UserResp {
        trx_id: String::from(""),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
