use async_trait::async_trait;

use crate::internal::user::entity::user::{
    UserAuthRequest, UserAuthResponse, UserChangePasswordRequest, UserCreateRequest, UserFromDb,
    UserGet, UserGetPassword, UserGetResponse, UserUpdateRequest, UserUpdateResponse
};
use crate::internal::user::usecase::repo::repo::UserRepo;
use crate::internal::controller::response;

#[derive(Clone)]
pub struct UserUseCase {
    pub repo: UserRepo
}

pub fn new_user_use_case(repo: UserRepo) -> UserUseCase {
    UserUseCase {
        repo
    }
}

#[async_trait]
pub trait UseCase {
    async fn user_auth(&self, user: UserAuthRequest) -> Result<UserAuthResponse, response::ErrorResponseUseCase>;
    async fn user_create(&self, user: UserCreateRequest) -> Result<UserGet, response::ErrorResponseUseCase>;
    async fn user_get_by_id(&self, id: i32) -> Result<UserGetResponse, response::ErrorResponseUseCase>;
    async fn user_get_by_username(&self, username: String) -> Result<UserGet, response::ErrorResponseUseCase>;
    async fn user_list(&self) -> Result<Vec<UserGetResponse>, response::ErrorResponseUseCase>;
    async fn user_update_by_id(&self, user: UserUpdateRequest) -> Result<UserUpdateResponse, response::ErrorResponseUseCase>;
    async fn user_change_password(&self, user: UserChangePasswordRequest) -> Result<(), response::ErrorResponseUseCase>;
}

#[async_trait]
pub trait Repo {
    async fn user_get_password_by_username(&self, username: String) -> Result<UserGetPassword, sqlx::Error>;
    async fn user_get_password_by_id(&self, id: i32) -> Result<UserGetPassword, sqlx::Error>;
    async fn user_create(&self, user: UserCreateRequest) -> Result<UserGet, sqlx::Error>;
    async fn user_get_by_id(&self, id: i32) -> Result<UserGetResponse, sqlx::Error>;
    async fn user_get_by_username(&self, username: String) -> Result<UserGet, sqlx::Error>;
    async fn user_list(&self) -> Result<Vec<UserGetResponse>, sqlx::Error>;
    async fn user_update_by_id(&self, user: UserUpdateRequest) -> Result<UserFromDb, sqlx::Error>;
    async fn user_change_password(&self, req: UserChangePasswordRequest) -> Result<(), sqlx::Error>;
}
