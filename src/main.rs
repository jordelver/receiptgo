use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use receiptgo::ringgo::errors::Error;
use receiptgo::ringgo::url_helpers;

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
            client_secret,
            grant_type: String::from("password"),
            username,
            password,
        }
    }
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DownloadRequest {
    identifier: String,
    resource_type_id: u8,
}

impl DownloadRequest {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            resource_type_id: 1,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DownloadRequestResponse {
    resource_access_token: String,
    return_code: u8,
    success: bool,
}

async fn get_authentication_token(
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

async fn retrieve_parking_sessions(access_token: &str) -> Result<Option<ParkingSessions>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url_helpers::parking_sessions_url())
        .bearer_auth(access_token)
        .send()
        .await?;

    Ok(Some(response.json::<ParkingSessions>().await?))
}

async fn request_receipt_pdf_download(
    access_token: &str,
    parking_session_id: &str,
) -> Result<String, Error> {
    let params = DownloadRequest::new(parking_session_id.to_owned());
    let client = reqwest::Client::new();
    let response = client
        .post(url_helpers::request_download_url())
        .bearer_auth(access_token)
        .json(&params)
        .send()
        .await?;

    let download_response = response.json::<DownloadRequestResponse>().await?;
    Ok(download_response.resource_access_token)
}

async fn download_receipt_pdf(access_token: &str, parking_session_id: String) -> Result<(), Error> {
    let download_token = request_receipt_pdf_download(access_token, &parking_session_id)
        .await
        .unwrap();
    let file_name = format!("{id}.pdf", id = parking_session_id);
    let response = reqwest::get(url_helpers::download_url(&download_token)).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let access_token = get_authentication_token(args.username, args.password, args.client_secret)
        .await
        .unwrap();

    let parking_sessions = retrieve_parking_sessions(&access_token).await.unwrap();

    if let Some(ps) = parking_sessions {
        let first_parking_session = ps.sessions.into_iter().next();

        let download_result =
            download_receipt_pdf(&access_token, first_parking_session.unwrap().auditlink).await;

        if download_result.is_ok() {
            println!("Downloaded receipt");
        } else {
            println!("Download failed");
        }
    } else {
        println!("No parking sessions");
    }
}
