use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Serialize, Deserialize};
use actix_web::http::header::ContentType;

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status_code: i32,
    pub error_msg: String,
}

pub struct ErrorResponseUseCase {
    pub error_msg: String,
    pub status_code: StatusCode,
}

pub fn convert_status_code_to_i32(code: StatusCode) -> i32 {
    match code {
        StatusCode::OK => {
            200
        },
        StatusCode::BAD_REQUEST => {
            400
        },
        StatusCode::UNAUTHORIZED => {
            401
        },
        StatusCode::NOT_FOUND => {
            404
        },
        StatusCode::CONFLICT => {
            409
        },
        _ => {
            500
        }
    }
}

pub fn send_error_response(err: ErrorResponseUseCase) -> HttpResponse {

    let response: GeneralResponse<ErrorResponse> = GeneralResponse {
        success: false,
        data: ErrorResponse {
            status_code: convert_status_code_to_i32(err.status_code),
            error_msg: err.error_msg,
        }
    };

    return match err.status_code {
        StatusCode::BAD_REQUEST => {
            HttpResponse::BadRequest().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
        },
        StatusCode::UNAUTHORIZED => {
            HttpResponse::Unauthorized().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
        }
        StatusCode::NOT_FOUND => {
            HttpResponse::NotFound().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
        },
        StatusCode::CONFLICT => {
            HttpResponse::Conflict().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
        }
        _ => {
            let response: GeneralResponse<ErrorResponse> = GeneralResponse {
                success: false,
                data: ErrorResponse {
                    status_code: 500,
                    error_msg: "internal server error".to_string(),
                }
            };
            HttpResponse::InternalServerError().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
        }
    }
}

pub fn send_success_response<T: Serialize>(data: T) -> HttpResponse {
    let response: GeneralResponse<T> = GeneralResponse {
        success: true,
        data,
    };

    HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&response).unwrap())
}
