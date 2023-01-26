use clap::Parser;
use serde::{Deserialize, Serialize};

use receiptgo::ringgo::errors::Error;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ParkingSessions {
    sessions: Vec<ParkingSession>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ParkingSession {
    #[serde(alias = "id")]
    auditlink: String,
}

#[derive(Deserialize, Serialize, Debug)]
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

impl AuthenticationRequest {
    pub fn new(username: String, password: String, client_secret: String) -> Self {
        Self {
            client_id: RINGGO_CLIENT_ID.to_string(),
            client_secret: client_secret,
            grant_type: String::from("password"),
            username: username,
            password: password,
        }
    }
}

async fn get_authentication_token(
    username: String,
    password: String,
    client_secret: String,
) -> String {
    let auth_params = AuthenticationRequest::new(username, password, client_secret);

    let client = reqwest::Client::new();

    let response = client.post(login_url()).form(&auth_params).send().await.unwrap();

    let auth_response = response.json::<AuthResponse>().await.unwrap();
    auth_response.access_token
}

async fn retrieve_parking_sessions(access_token: String) -> Result<Option<ParkingSessions>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(parking_sessions_url())
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ParkingSessions>().await {
                Ok(parsed) => Ok(Some(parsed)),
                Err(_) => panic!("Error parsing JSON"),
            }
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            Err(Error::Unauthorized)
        }
        _ => {
            Err(Error::Unknown)
        }
    }
}

fn login_url() -> String {
    format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/auth/v1/pword"
    )
}

fn parking_sessions_url() -> String {
    format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/user/sessions/receipts/1?CountryCode=GB"
    )
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let access_token =
        get_authentication_token(args.username, args.password, args.client_secret).await;

    let parking_sessions = retrieve_parking_sessions(access_token).await.unwrap();

    if let Some(ps) = parking_sessions {
        println!("Parking sessions");
        for session in ps.sessions.into_iter() {
            println!("- {}", session.auditlink);
        }
    } else {
        println!("No parking sessions");
    }
}
