static RINGGO_API_BASE_URL: &str = "https://api-blue.myringgo.co.uk";

pub fn login_url() -> String {
    format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/auth/v1/pword"
    )
}

pub fn parking_sessions_url() -> String {
    format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/user/sessions/receipts/1?CountryCode=GB"
    )
}

pub fn request_download_url() -> String {
    format!(
        "{api_url}{path}",
        api_url = RINGGO_API_BASE_URL,
        path = "/resource/accesstoken"
    )
}

pub fn download_url(parking_session_id: &str) -> String {
    format!(
        "{api_url}{path}{parking_session_id}",
        api_url = RINGGO_API_BASE_URL,
        path = "/user/session/receipt/",
        parking_session_id = parking_session_id,
    )
}
