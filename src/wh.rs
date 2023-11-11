use crate::user::update_user;
use actix_web::web::Bytes;
use actix_web::{http::header::ContentType, HttpResponse};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct HookResp {
    message: String,
}

pub async fn webhook(body: Bytes) -> HttpResponse {
    let json_string = String::from_utf8_lossy(&body);

    let parsed_json =
        serde_json::from_str(&format!("{}", json_string)).expect("Failed to parse JSON");

    let event_type_path = "attributes.name";
    let event_type =
        extract_value(&parsed_json, event_type_path).unwrap_or_else(|| return String::from(""));
    let event_type_str: &str = event_type.trim_matches('"');

    let reference_id_path = "attributes.payload.data.attributes.reference-id";
    let reference_id =
        extract_value(&parsed_json, reference_id_path).unwrap_or_else(|| return String::from(""));
    let reference_id_str: &str = reference_id.trim_matches('"');

    let country_code_path = "attributes.payload.data.attributes.fields.address-country-code.value";
    let country_code =
        extract_value(&parsed_json, country_code_path).unwrap_or_else(|| return String::from(""));
    let country_code_str: &str = country_code.trim_matches('"');

    if reference_id_str.is_empty() || reference_id_str == "null" {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{}");
    }

    if event_type_str == "inquiry.completed" {
        let _ = update_user(
            reference_id_str.to_owned(),
            String::from("invest_state"),
            String::from("PENDING_KYC"),
        );
    } else if event_type_str == "inquiry.approved" {
        let _ = update_user(
            reference_id_str.to_owned(),
            String::from("invest_state"),
            String::from("KYC_VERIFIED"),
        );

        let mut default_country_code = String::from("NON_US_INDIVIDUAL");
        if country_code_str == "US" {
            default_country_code = String::from("US_ENTITY");
        }

        let _ = update_user(
            reference_id_str.to_owned(),
            String::from("user_type"),
            default_country_code,
        );
    }

    let resp = HookResp {
        message: String::from("Ok"),
    };

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(resp)
}

fn extract_value(json: &Value, path: &str) -> Option<String> {
    let mut current_value = json;

    for part in path.split('.') {
        match current_value.get(part) {
            Some(value) => {
                current_value = value;
            }
            None => return None,
        }
    }

    Some(current_value.to_string())
}
