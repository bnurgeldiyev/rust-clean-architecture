use actix_web::web::Data;
use crate::pkg::postgres::connection::Db;

#[derive(Clone)]
pub struct UserRepo {
    pub db: Data<Db>,
}

pub fn new_user_repo(db: Data<Db>) -> UserRepo {
    UserRepo{
        db
    }
}
