use clap::Parser;

/// Download receipts from RingGo
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, required = true, env = "RINGGO_TOKEN")]
    pub token: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
