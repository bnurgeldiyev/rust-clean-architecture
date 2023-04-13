use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::config::config::Config;

pub type Db = Pool<Postgres>;

pub async fn new_pg_connection(cfg: &Config) -> Result<Db, sqlx::Error> {
    let conn_string = format!("postgres://{}:{}@{}/{}", cfg.db_user, cfg.db_password, cfg.db_host, cfg.db_name);
    PgPoolOptions::new()
        .max_connections(cfg.db_max_conn)
        .connect(&conn_string)
        .await
}
