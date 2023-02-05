use clap::Parser;

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
