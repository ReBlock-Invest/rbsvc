use actix_web::{http::header::ContentType, web, HttpResponse};
use ethers::prelude::*;
use ethers::utils::keccak256;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;

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
    let redis_url = env::var("REDIS_URL").expect("missing environment variable REDIS_URL");

    redis::Client::open(redis_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub async fn nonce(data: web::Json<NonceReq>) -> HttpResponse {
    let mut conn = connect();

    let key = format!("rb:nonce:{}", data.address);
    let n_data: u64 = conn.get(&key).unwrap();

    let nonce = NonceResp {
        nonce: n_data,
        address: data.address.to_owned(),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(nonce)
}

pub async fn wallet_login(data: web::Json<LoginReq>) -> HttpResponse {
    let mut conn = connect();
    let key = format!("rb:nonce:{}", data.address);
    let n_data: u64 = conn.get(&key).unwrap();

    let signature = Signature::from_str(&data.signature).expect("Invalid signature format");
    let message = eth_message(n_data.to_string());
    let address = Address::from_str(&data.address).expect("Invalid Ethereum address format");

    let is_valid = signature
        .verify(n_data.to_string().as_str(), address)
        .is_ok();

    if !is_valid {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{}");
    }

    if let Ok(recovered_public_key) = signature.recover(message) {
        let rec_address = Address::from(recovered_public_key);

        if rec_address != address {
            return HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("{}");
        }
    } else {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{}");
    }

    let resp = LoginResp {
        access_token: String::from("AaaJhHHJ"),
    };

    let _: i64 = conn.incr(&key, 1).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}

fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}
