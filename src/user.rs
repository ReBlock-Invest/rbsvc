use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use eyre::Result;
use redis::Commands;
use serde::Serialize;
use std::env;

#[derive(Serialize, Debug)]
pub struct UserResp {
    pub invest_state: String,
    pub user_type: String,
    pub address: String,
}

fn connect() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("missing environment variable REDIS_URL");

    redis::Client::open(redis_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub async fn user_info(req: HttpRequest) -> HttpResponse {
    let mut user_address = String::from("");
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(token) = auth_header.to_str() {
            let mut conn = connect();
            let session_key = format!("rb:session:{}", token);

            user_address = conn.get(&session_key).unwrap_or_else(|_err| {
                return String::from("");
            });

            if user_address.is_empty() {
                return HttpResponse::Unauthorized()
                    .content_type(ContentType::json())
                    .body("{\"error\": \"Invalid auth header\"}");
            }
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

    let resp = get_user(user_address).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}

pub fn update_user(address: String, field: String, value: String) -> Result<()> {
    let mut conn = connect();
    let key = format!("rb:user:{}", address);

    let _: () = conn
        .hset(key.clone(), field, value)
        .expect("Failed to set field in hash");

    let _: () = conn
        .hset(key, "address", address)
        .expect("Failed to set field in hash");

    Ok(())
}

pub fn get_user(address: String) -> Result<UserResp> {
    let mut conn = connect();
    let key = format!("rb:user:{}", address);

    let invest_state = conn.hget(&key, "invest_state").unwrap_or_else(|_err| {
        return String::from("");
    });

    let user_type = conn.hget(&key, "user_type").unwrap_or_else(|_err| {
        return String::from("");
    });

    let user = UserResp {
        invest_state: invest_state,
        user_type: user_type,
        address: address,
    };

    Ok(user)
}
