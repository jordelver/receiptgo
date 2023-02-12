use crate::ringgo::errors::Error;
use crate::ringgo::url_helpers;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

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

pub async fn download_receipt_pdf(
    access_token: &str,
    parking_session_id: String,
) -> Result<(), Error> {
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
