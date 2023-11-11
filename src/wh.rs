use crate::user::{get_user, update_user};
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct HookReq {
    address: String,
}

#[derive(Serialize, Debug)]
pub struct HookResp {
    message: String,
}

pub async fn webhook(req: HttpRequest, data: web::Json<HookReq>) -> HttpResponse {
    update_user_invest_state(&data.address);
    update_user_type(&data.address);
    let resp = HookResp {
        message: String::from("Ok"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}

fn update_user_invest_state(address: &str) {
    let user = get_user(address.to_owned()).unwrap();

    if user.invest_state == String::from("WALLET_VERIFIED") {
        let _ = update_user(
            address.to_owned(),
            String::from("invest_state"),
            String::from("PENDING_KYC"),
        );
    } else if user.invest_state == String::from("PENDING_KYC") {
        let _ = update_user(
            address.to_owned(),
            String::from("invest_state"),
            String::from("KYC_VERIFIED"),
        );
    }
}

fn update_user_type(address: &str) {
    let user = get_user(address.to_owned()).unwrap();

    if user.user_type.is_empty() {
        let _ = update_user(
            address.to_owned(),
            String::from("user_type"),
            String::from("NON_US_INDIVIDUAL"),
        );
    }
}
