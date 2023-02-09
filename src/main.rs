use clap::Parser;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use receiptgo::cli;
use receiptgo::ringgo::authentication;
use receiptgo::ringgo::errors::Error;
use receiptgo::ringgo::url_helpers;

/// Number of receipts to download
static RECEIPTS_TO_DOWNLOAD: usize = 5;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ParkingSessions {
    sessions: Vec<ParkingSession>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ParkingSession {
    #[serde(alias = "Auditlink")]
    id: String,
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
    let file_name = format!("{parking_session_id}.pdf");
    let response = reqwest::get(url_helpers::download_url(&download_token)).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    let access_token = authentication::get_authentication_token(args.username, args.password, args.client_secret)
        .await
        .unwrap();

    let parking_sessions = retrieve_parking_sessions(&access_token).await.unwrap();

    if let Some(ps) = parking_sessions {
        for session in ps.sessions.into_iter().take(RECEIPTS_TO_DOWNLOAD) {
            println!("Downloading {}", session.id);

            let download_result = download_receipt_pdf(&access_token, session.id).await;

            if download_result.is_ok() {
                println!("> Downloaded");
            } else {
                println!("> Download failed");
            }

            println!();
        }
    } else {
        println!("No parking sessions");
    }
}
