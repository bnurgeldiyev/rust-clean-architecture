use dotenv::dotenv;

pub struct Config {
    pub http_host: String,
    pub http_port: String,

    pub db_host: String,
    pub db_port: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    pub db_max_conn: u32,
}

pub fn read_env() -> Config{
    dotenv().ok();

    Config {
        http_host: std::env::var("HTTP_HOST").expect("HTTP_HOST must be set."),
        http_port: std::env::var("HTTP_PORT").expect("HTTP_PORT must be set."),
        db_host: std::env::var("DB_HOST").expect("DB_HOST must be set."),
        db_port: std::env::var("DB_PORT").expect("DB_PORT must be set."),
        db_user: std::env::var("DB_USER").expect("DB_USER must be set."),
        db_password: std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set."),
        db_name: std::env::var("DB_NAME").expect("DB_NAME must be set."),
        db_max_conn: std::env::var("DB_MAX_CONN").expect("DB_MAX_CONN must be set.").trim().parse().expect("can't convert to u32"),
    }
}
