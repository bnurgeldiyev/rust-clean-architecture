#![allow(deprecated)]

use std::io;
use actix_web;
use actix_web::{App, HttpServer, web};

use crate::config::config::read_env;
use crate::pkg::postgres::connection;
use crate::internal::user::usecase::repo::repo::new_user_repo;
use crate::internal::user::usecase::traits::{new_user_use_case, UserUseCase};
use crate::internal::controller::user_controller::user_routes;

mod config;
mod internal;
mod pkg;

#[derive(Clone)]
pub struct UseCases {
    user_use_case: UserUseCase,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let cfg = read_env();

    let db = match connection::new_pg_connection(&cfg).await {
        Ok(database) => {
            database
        }
        Err(err) => {
            panic!("Error in new_pg_connection: {}", err)
        }
    };

    let db = web::Data::new(db);
    let user_repo = new_user_repo(db);
    let user_use_case = new_user_use_case(user_repo);

    let use_cases = UseCases {
        user_use_case
    };

    println!("<--START-SERVER--> {} {}", cfg.http_host, cfg.http_port);
    HttpServer::new(move || {
        App::new()
            .data(use_cases.clone())
            .service(user_routes::user_auth)
            .service(user_routes::user_create)
            .service(user_routes::user_list)
            .service(user_routes::user_get)
            .service(user_routes::user_update_by_id)
            .service(user_routes::user_change_password)
    })
        .bind((cfg.db_host, 8081))?
        .run()
        .await
}
