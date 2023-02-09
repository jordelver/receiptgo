use crate::ringgo::errors::Error;
use crate::ringgo::url_helpers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ParkingSessions {
    pub sessions: Vec<ParkingSession>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ParkingSession {
    #[serde(alias = "Auditlink")]
    pub id: String,
}

pub async fn retrieve_parking_sessions(
    access_token: &str,
) -> Result<Option<ParkingSessions>, Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(url_helpers::parking_sessions_url())
        .bearer_auth(access_token)
        .send()
        .await?;

    Ok(Some(response.json::<ParkingSessions>().await?))
}
