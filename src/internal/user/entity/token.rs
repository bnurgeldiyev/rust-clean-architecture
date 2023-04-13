use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use jwt::{SignWithKey};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub const TOKEN_SECRET_KEY: &str = "bnurgeldiyev514";
const TOKEN_LIFE_TIME: u64 = 5; // minute

pub fn get_time_sec() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

pub fn generate_access_token(username: &String) -> String {

    let key: Hmac<Sha256> = Hmac::new_from_slice(TOKEN_SECRET_KEY.as_ref()).unwrap();
    let mut claims = BTreeMap::new();
    let start = get_time_sec() + (TOKEN_LIFE_TIME * 60);
    let start_str = start.to_string().clone();
    claims.insert("username", username);
    claims.insert("created_time", &start_str);

    let token_str = claims.sign_with_key(&key).unwrap();

    token_str
}
