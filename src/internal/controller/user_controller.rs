pub mod user_routes {
    use actix_web::{Responder, web, post, get, put, HttpRequest};
    use actix_web::http::StatusCode;
    use crate::internal::user::entity::user;
    use crate::internal::user::usecase::traits::UseCase;
    use crate::internal::controller::response::{ErrorResponseUseCase, send_error_response, send_success_response};
    use crate::internal::controller::middleware;
    use crate::internal::user::entity::user::{verify_user_auth_request, verify_user_create_request};

    #[post("/api/v1/user/auth")]
    pub async fn user_auth(
        body: String,
        use_cases: web::Data<crate::UseCases>,
    ) -> impl Responder {
        let des: user::UserAuthRequest = match serde_json::from_str(&body) {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: "Can't convert request".to_string(),
                };

                return send_error_response(res);
            }
        };

        return match verify_user_auth_request(des.clone()) {
            Ok(_res) => {
                match use_cases.user_use_case.user_auth(des).await {
                    Ok(res) => {
                        send_success_response(res)
                    }
                    Err(err) => {
                        send_error_response(err)
                    }
                }
            }
            Err(err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: err.to_string(),
                };

                send_error_response(res)
            }
        };
    }

    #[post("/api/v1/user/password-change")]
    pub async fn user_change_password(
        body: String,
        use_cases: web::Data<crate::UseCases>,
    ) -> impl Responder {
        let des: user::UserChangePasswordRequest = match serde_json::from_str(&body) {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: "Can't convert request".to_string(),
                };

                return send_error_response(res);
            }
        };

        return match use_cases.user_use_case.user_change_password(des).await {
            Ok(res) => {
                send_success_response(res)
            }
            Err(err) => {
                send_error_response(err)
            }
        };
    }

    #[put("/api/v1/user/update")]
    pub async fn user_update_by_id(
        req: HttpRequest,
        body: String,
        use_cases: web::Data<crate::UseCases>,
    ) -> impl Responder {
        let _token_result = match middleware::is_unauthorized(&req).await {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::UNAUTHORIZED,
                    error_msg: "Unauthorized".to_string(),
                };

                return send_error_response(res);
            }
        };

        let des: user::UserUpdateRequest = match serde_json::from_str(&body) {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: "Invalid request".to_string(),
                };

                return send_error_response(res);
            }
        };

        return match use_cases.user_use_case.user_update_by_id(des).await {
            Ok(res) => {
                send_success_response(res)
            }
            Err(err) => {
                send_error_response(err)
            }
        };
    }

    #[post("/api/v1/user/create")]
    pub async fn user_create(
        req: HttpRequest,
        body: String,
        use_cases: web::Data<crate::UseCases>,
    ) -> impl Responder {
        let _token_result = match middleware::is_unauthorized(&req).await {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::UNAUTHORIZED,
                    error_msg: "Unauthorized".to_string(),
                };

                return send_error_response(res);
            }
        };

        let des: user::UserCreateRequest = match serde_json::from_str(&body) {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: "Invalid request".to_string(),
                };

                return send_error_response(res);
            }
        };

        return match verify_user_create_request(des.clone()) {
            Ok(_res) => {
                match use_cases.user_use_case.user_create(des).await {
                    Ok(res) => {
                        send_success_response(res)
                    }
                    Err(err) => {
                        send_error_response(err)
                    }
                }
            }
            Err(err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::BAD_REQUEST,
                    error_msg: err.to_string(),
                };

                send_error_response(res)
            }
        };
    }

    #[get("/api/v1/user/{id}/get")]
    pub async fn user_get(
        req: HttpRequest,
        id: web::Path<i32>,
        use_cases: web::Data<crate::UseCases>,
    ) -> impl Responder {
        let _token_result = match middleware::is_unauthorized(&req).await {
            Ok(res) => {
                res
            }
            Err(_err) => {
                let res = ErrorResponseUseCase {
                    status_code: StatusCode::UNAUTHORIZED,
                    error_msg: "Unauthorized".to_string(),
                };

                return send_error_response(res);
            }
        };

        let id = id.into_inner();

        return match use_cases.user_use_case.user_get_by_id(id).await {
            Ok(res) => {
                send_success_response(res)
            }
            Err(err) => {
                send_error_response(err)
            }
        };
    }

    #[get("/api/v1/user/list")]
    pub async fn user_list(
        use_cases: web::Data<crate::UseCases>
    ) -> impl Responder {
        return match use_cases.user_use_case.user_list().await {
            Ok(res) => {
                send_success_response(res)
            }
            Err(err) => {
                send_error_response(err)
            }
        };
    }
}
