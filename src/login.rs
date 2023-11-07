use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Debug)]
pub struct NonceReq {
    address: String,
}

#[derive(Serialize, Debug)]
pub struct NonceResp {
    nonce: u64,
    address: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    address: String,
    signature: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResp {
    access_token: String,
}

fn connect() -> redis::Connection {
    //format - host:port
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default(); //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    println!("{}", redis_conn_url);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub async fn nonce(form: web::Json<NonceReq>) -> HttpResponse {
    let nonce = NonceResp {
        nonce: 1,
        address: form.address.to_owned(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(nonce)
}

pub async fn wallet_login(data: web::Json<LoginReq>) -> HttpResponse {
    println!("{:?}", data.address);
    println!("{:?}", data.signature);

    let resp = LoginResp {
        access_token: String::from("AaaJhHHJ"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
