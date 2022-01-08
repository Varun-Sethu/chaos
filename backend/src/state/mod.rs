use dotenv_codegen::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::Client;
use serde_json::Value;

const GOOGLE_OPENID_DISCOVERY_URL: &str =
    "https://accounts.google.com/.well-known/openid-configuration";
const GOOGLE_OPENID_USERINFO_KEY: &str = "userinfo_endpoint";

pub struct ApiState {
    pub reqwest_client: Client,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey<'static>,
    pub userinfo_endpoint: String,
}

pub async fn api_state() -> ApiState {
    let reqwest_client = Client::new();

    let jwt_secret = dotenv!("JWT_SECRET");

    let discovery = reqwest::get(GOOGLE_OPENID_DISCOVERY_URL)
        .await
        .expect(&format!(
            "Failed to fetch openid discovery from {:?}",
            GOOGLE_OPENID_DISCOVERY_URL
        ))
        .json::<serde_json::Map<String, Value>>()
        .await
        .expect("Failed to parse openid discovery as a JSON Object");

    let userinfo_endpoint = match discovery.get(GOOGLE_OPENID_USERINFO_KEY).expect(&format!(
        "Openid discovery does not contain a {:?} key",
        GOOGLE_OPENID_USERINFO_KEY
    )) {
        Value::String(url) => url.to_string(),
        other => panic!(
            "Openid discovery {:?} key has incorrect type {:?}",
            GOOGLE_OPENID_USERINFO_KEY, other
        ),
    };

    let api_state = ApiState {
        reqwest_client,
        jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
        jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
        userinfo_endpoint,
    };

    api_state
}