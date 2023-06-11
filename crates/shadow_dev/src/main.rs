use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[clap(long, default_value = "0.0.0.0")]
    host: std::net::IpAddr,

    #[clap(long, default_value = "8080")]
    port: u16,

    #[clap(long, default_value = "0.0.0.0")]
    child_host: std::net::IpAddr,

    #[clap(long)]
    command: String,
}

#[tokio::main]
pub async fn main() {
    let Args {
        host,
        port,
        child_host,
        command,
    } = Args::parse();
}
