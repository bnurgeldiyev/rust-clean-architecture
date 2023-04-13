use async_trait::async_trait;
use sqlx::Error;

use crate::internal::user::entity::user::{
    convert_unix_to_date, UserChangePasswordRequest, UserCreateRequest, UserEmpty,
    UserFromDb, UserGet, UserGetPassword, UserGetResponse, UserUpdateRequest
};
use crate::internal::user::usecase::repo::repo::UserRepo;
use crate::internal::user::usecase::traits::Repo;
use crate::internal::user::entity::token::get_time_sec;

#[async_trait]
impl Repo for UserRepo {
    async fn user_get_password_by_username(&self, username: String) -> Result<UserGetPassword, Error> {
        let sql = "SELECT password FROM tbl_user WHERE username=$1";
        let query = sqlx::query_as::<_, UserGetPassword>(&sql).bind(username);

        return match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                Err(err)
            }
        };
   }

    async fn user_get_password_by_id(&self, id: i32) -> Result<UserGetPassword, Error> {
        let sql = "SELECT password FROM tbl_user WHERE id=$1";
        let query = sqlx::query_as::<_, UserGetPassword>(&sql).bind(id);

        return match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                Err(err)
            }
        };
    }

    async fn user_create(&self, user: UserCreateRequest) -> Result<UserGet, Error> {
        let sql = "INSERT INTO tbl_user(username, password, firstname, lastname) VALUES($1, $2, $3, $4)\
         RETURNING id, username, firstname, lastname";

        let query = sqlx::query_as::<_, UserGet>(&sql)
            .bind(user.username)
            .bind(user.password)
            .bind(user.firstname)
            .bind(user.lastname);

        return match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                Err(err)
            }
        };
    }

    async fn user_get_by_id(&self, id: i32) -> Result<UserGetResponse, Error> {
        let sql = "SELECT id, username, firstname, lastname, create_ts, update_ts FROM tbl_user WHERE id=$1";
        let query = sqlx::query_as::<_, UserFromDb>(&sql).bind(id);

        let res = match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Some(data)
            },
            Err(_err) => {
                None
            }
        };

        return match res {
            Some(res) => {
                let data = UserGetResponse {
                    id,
                    username: res.username,
                    firstname: res.firstname,
                    lastname: res.lastname,
                    create_ts: convert_unix_to_date(res.create_ts),
                    update_ts: convert_unix_to_date(res.update_ts),
                };

                Ok(data)
            },
            None => {
                Err(Error::RowNotFound)
            }
        }
    }

    async fn user_get_by_username(&self, username: String) -> Result<UserGet, Error> {
        let sql = "SELECT id, username, firstname, lastname FROM tbl_user WHERE username=$1";
        let query = sqlx::query_as::<_, UserGet>(&sql).bind(username);

        return match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                Err(err)
            }
        };
    }

    async fn user_list(&self) -> Result<Vec<UserGetResponse>, Error> {
        let sql = "SELECT id, username, firstname, lastname, create_ts, update_ts FROM tbl_user ORDER BY id";
        let query = sqlx::query_as::<_, UserFromDb>(&sql);

        let users = match query.fetch_all(&**self.db).await {
            Ok(data) => {
                Some(data)
            },
            Err(_err) => {
                None
            }
        };

        return match users {
            Some(data) => {
                let mut users: Vec<UserGetResponse> = Vec::new();

                for user in data {
                    let u = UserGetResponse {
                        id: user.id,
                        username: user.username,
                        firstname: user.firstname,
                        lastname: user.lastname,
                        create_ts: convert_unix_to_date(user.create_ts),
                        update_ts: convert_unix_to_date(user.update_ts),
                    };

                    users.push(u);
                }

                Ok(users)
            },
            None => {
                Err(Error::RowNotFound)
            }
        };
    }

    async fn user_update_by_id(&self, user: UserUpdateRequest) -> Result<UserFromDb, Error> {
        let sql = "UPDATE tbl_user SET username=$1, firstname=$2, lastname=$3, update_ts=$4 WHERE id=$5\
        RETURNING id, username, firstname, lastname, create_ts, update_ts";
        let query = sqlx::query_as::<_, UserFromDb>(&sql)
            .bind(user.username)
            .bind(user.firstname)
            .bind(user.lastname)
            .bind(get_time_sec() as i64)
            .bind(user.id);

        return match query.fetch_one(&**self.db).await {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    async fn user_change_password(&self, req: UserChangePasswordRequest) -> Result<(), Error> {
        let sql = "UPDATE tbl_user SET password=$1, update_ts=$2 WHERE id=$3 RETURNING id";

        let query = sqlx::query_as::<_, UserEmpty>(&sql)
            .bind(req.new_password)
            .bind(get_time_sec() as i64)
            .bind(req.id);

        return match query.fetch_one(&**self.db).await {
            Ok(_data) => {
                Ok(())
            },
            Err(err) => {
                Err(err)
            }
        }
    }
}
