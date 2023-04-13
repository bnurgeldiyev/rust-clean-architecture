use actix_web::http::StatusCode;
use async_trait::async_trait;
use bcrypt::{hash, verify};
use uuid::Uuid;

use crate::internal::user::entity::user::{
    convert_unix_to_date, UserAuthRequest, UserAuthResponse, UserChangePasswordRequest,
    UserCreateRequest, UserGet, UserGetResponse, UserUpdateRequest, UserUpdateResponse
};

use crate::internal::user::usecase::traits::{Repo, UseCase, UserUseCase};
use crate::internal::user::entity::token;
use crate::internal::controller::response::ErrorResponseUseCase;

#[async_trait]
impl UseCase for UserUseCase {
    async fn user_auth(&self, user: UserAuthRequest) -> Result<UserAuthResponse, ErrorResponseUseCase> {

        let user_by_username: Option<UserGet> = match self.repo.user_get_by_username(user.username.clone()).await {
            Ok(res) => {
                Some(res)
            }

            Err(err) => {
                if err.to_string() != sqlx::Error::RowNotFound.to_string() {
                    let res = ErrorResponseUseCase{
                        error_msg: "Error in repo.user_get_by_username".to_string(),
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    };

                    return Err(res);
                }

                None
            }
        };

        match user_by_username {
            Some(_data) => {
                let password_by_username = match self.repo.user_get_password_by_username(user.username.clone()).await {
                    Ok(data) => {
                        data
                    },
                    Err(_err) => {
                        let res = ErrorResponseUseCase{
                            error_msg: "Error in repo.user_get_password_by_username".to_string(),
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        };

                        return Err(res);
                    }
                };

                let valid = verify(user.password.clone(), &password_by_username.password).unwrap();
                
                if valid {
                    let access_token = token::generate_access_token(&user.username);
                    let refresh_token = Uuid::new_v4().to_string();

                    let res: UserAuthResponse = UserAuthResponse {
                        access_token,
                        refresh_token
                    };
                    
                    Ok(res)
                } else {
                    let res = ErrorResponseUseCase{
                        error_msg: "Unauthorized".to_string() ,
                        status_code: StatusCode::UNAUTHORIZED,
                    };

                    Err(res)
                }
            },
            None => {
                let res = ErrorResponseUseCase{
                    error_msg: "Unauthorized".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                };

                Err(res)
            }
        }
    }

    async fn user_create(&self, mut user: UserCreateRequest) -> Result<UserGet, ErrorResponseUseCase> {

        let user_by_username: Option<UserGet> = match self.repo.user_get_by_username(user.username.clone()).await {
            Ok(res) => {
                Some(res)
            }

            Err(err) => {
                if err.to_string() != sqlx::Error::RowNotFound.to_string() {
                    let data = ErrorResponseUseCase {
                        status_code: StatusCode::INTERNAL_SERVER_ERROR,
                        error_msg: "Internal server error".to_string(),
                    };

                    return Err(data);
                }

                None
            }
        };

        match user_by_username {
            Some(_data) => {
                let data = ErrorResponseUseCase {
                    status_code: StatusCode::CONFLICT,
                    error_msg: format!("User with username={} already exists", user.username.clone()),
                };

                Err(data)
            },
            None => {
                let hashed = hash(user.password.clone(), 12).unwrap();
                user.password = hashed;

                let res = match self.repo.user_create(user).await {
                    Ok(data) => {
                        data
                    },
                    Err(_err) => {
                        let data = ErrorResponseUseCase {
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            error_msg: "Internal server error".to_string(),
                        };

                        return Err(data);
                    }
                };

                Ok(res)
            }
        }
    }

    async fn user_get_by_id(&self, id: i32) -> Result<UserGetResponse, ErrorResponseUseCase> {
        match self.repo.user_get_by_id(id).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {

                if err.to_string() == sqlx::Error::RowNotFound.to_string() {
                    let res = ErrorResponseUseCase {
                        status_code: StatusCode::NOT_FOUND,
                        error_msg: format!("User with id={} not found", id),
                    };
                    return Err(res);
                }

                let res = ErrorResponseUseCase {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_msg: "Internal server error".to_string(),
                };

                Err(res)
            }
        }
    }

    async fn user_get_by_username(&self, username: String) -> Result<UserGet, ErrorResponseUseCase> {
        match self.repo.user_get_by_username(username.clone()).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                let mut data = ErrorResponseUseCase{
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_msg: "Internal server error".to_string(),
                };

                if err.to_string() == sqlx::Error::RowNotFound.to_string() {
                    data.status_code = StatusCode::NOT_FOUND;
                    data.error_msg = format!("User with username={} not found", username);
                    return Err(data);
                }

                Err(data)
            }
        }
    }

    async fn user_list(&self) -> Result<Vec<UserGetResponse>, ErrorResponseUseCase> {
        match self.repo.user_list().await {
            Ok(data) => {
                Ok(data)
            },
            Err(_err) => {
                let res = ErrorResponseUseCase{
                    error_msg: "Internal server error".to_string(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                };

                Err(res)
            }
        }
    }

    async fn user_update_by_id(&self, user: UserUpdateRequest) -> Result<UserUpdateResponse, ErrorResponseUseCase> {

        let user_by_id = match self.repo.user_get_by_id(user.id).await {
            Ok(res) => {
                Some(res)
            },
            Err(_err) => {
                None
            }
        };

        match user_by_id {
            Some(_res) => {

                match self.repo.user_update_by_id(user).await {
                    Ok(res) => {

                        let response = UserUpdateResponse {
                            id: res.id,
                            username: res.username,
                            firstname: res.firstname,
                            lastname: res.lastname,
                            create_ts: convert_unix_to_date(res.create_ts),
                            update_ts: convert_unix_to_date(res.update_ts),
                        };

                        Ok(response)
                    },
                    Err(_err) => {

                        let data = ErrorResponseUseCase {
                            status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            error_msg: "Internal server error".to_string(),
                        };

                        return Err(data)
                    }
                }

            },
            None => {
                let data = ErrorResponseUseCase {
                    status_code: StatusCode::NOT_FOUND,
                    error_msg: format!("User with id={} not found", user.id),
                };

                Err(data)
            }
        }
    }

    async fn user_change_password(&self, mut user: UserChangePasswordRequest) -> Result<(), ErrorResponseUseCase> {

        let password_by_id = match self.repo.user_get_password_by_id(user.id).await {
            Ok(data) => {
                Some(data)
            },
            Err(_err) => {
                None
            }
        };

        match password_by_id {
            Some(data) => {
                let valid = match verify(user.old_password.clone(), &data.password) {
                    Ok(_data) => {
                        true
                    },
                    Err(_err) => {
                        false
                    }
                };

                if valid {
                    let hashed = hash(user.new_password.clone(), 12).unwrap();
                    user.new_password = hashed;

                    return match self.repo.user_change_password(user).await {
                        Ok(_) => {
                            Ok(())
                        },
                        Err(_err) => {
                            let data = ErrorResponseUseCase {
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                                error_msg: "Internal server error".to_string(),
                            };

                            Err(data)
                        }
                    }

                } else {
                    let data = ErrorResponseUseCase {
                        status_code: StatusCode::BAD_REQUEST,
                        error_msg: "Invalid old password".to_string(),
                    };

                    Err(data)
                }

            },
            None => {
                let data = ErrorResponseUseCase {
                    status_code: StatusCode::NOT_FOUND,
                    error_msg: format!("User with id={} not found", user.id),
                };
                Err(data)
            }
        }
    }
}
