use clap::Parser;
use serde::{Deserialize, Serialize};

static RINGGO_API_BASE_URL: &str = "https://api-blue.myringgo.co.uk";
static RINGGO_CLIENT_ID: &str = "ringgoios";

/// Download receipts from RingGo
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, env = "RINGGO_USERNAME")]
    pub username: String,

    #[clap(short, long, env = "RINGGO_PASSWORD")]
    pub password: String,

    #[clap(short, long, env = "RINGGO_CLIENT_SECRET")]
    pub client_secret: String,
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i32,
    refresh_token: String,
}

#[derive(Default, Serialize, Debug)]
struct AuthenticationRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
    password: String,
    username: String,
}

async fn get_authentication_token(
    username: String,
    password: String,
    client_secret: String,
) -> String {
    let auth_params = AuthenticationRequest {
        client_id: RINGGO_CLIENT_ID.to_string(),
        client_secret: client_secret.to_string(),
        grant_type: String::from("password"),
        username: username,
        password: password,
    };

    let url = format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/auth/v1/pword"
    );

    let client = reqwest::Client::new();

    let response = client.post(url).form(&auth_params).send().await.unwrap();

    let auth_response = response.json::<AuthResponse>().await.unwrap();
    auth_response.access_token
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let access_token =
        get_authentication_token(args.username, args.password, args.client_secret).await;

    println!("{}", access_token);

}
