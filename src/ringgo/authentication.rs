use crate::ringgo::errors::Error;
use crate::ringgo::url_helpers;
use serde::{Deserialize, Serialize};

static RINGGO_CLIENT_ID: &str = "ringgoios";

#[derive(Default, Serialize, Debug)]
struct AuthenticationRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
    password: String,
    username: String,
}

impl AuthenticationRequest {
    pub fn new(username: String, password: String, client_secret: String) -> Self {
        Self {
            client_id: RINGGO_CLIENT_ID.to_string(),
            client_secret,
            grant_type: String::from("password"),
            username,
            password,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    refresh_token: String,
}

pub async fn get_token(
    username: String,
    password: String,
    client_secret: String,
) -> Result<String, Error> {
    let auth_params = AuthenticationRequest::new(username, password, client_secret);
    let client = reqwest::Client::new();
    let response = client
        .post(url_helpers::login_url())
        .form(&auth_params)
        .send()
        .await
        .unwrap();

    let auth_response = response.json::<AuthResponse>().await.unwrap();
    Ok(auth_response.access_token)
}
