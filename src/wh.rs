use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use eyre::Result;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct UserReq {
    recipient: String,
}

#[derive(Serialize, Debug)]
pub struct UserResp {
    trx_id: String,
}

pub async fn webhook(req: HttpRequest, data: web::Json<UserReq>) -> HttpResponse {
    let resp = UserResp {
        trx_id: String::from(""),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}
