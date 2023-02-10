use clap::Parser;

use receiptgo::cli;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    receiptgo::run(args).await
}
