pub mod cli;
pub mod ringgo;

use ringgo::{authentication, downloads, parking_sessions};

/// Number of receipts to download
static RECEIPTS_TO_DOWNLOAD: usize = 5;

pub async fn run(args: cli::Args) {
    let access_token = authentication::get_token(args.username, args.password, args.client_secret)
        .await
        .unwrap();

    let parking_sessions = parking_sessions::retrieve_parking_sessions(&access_token)
        .await
        .unwrap();

    if let Some(ps) = parking_sessions {
        for session in ps.sessions.into_iter().take(RECEIPTS_TO_DOWNLOAD) {
            println!("{:?}", session);
            println!("Downloading {}", session.id);

            let download_result = downloads::download_receipt_pdf(&access_token, session.id).await;

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
