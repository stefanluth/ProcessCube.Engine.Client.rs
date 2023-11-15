use clap::Parser;
use processcube_engine_client::cli::client::{register_commands, Cli};

#[tokio::main]
async fn main() -> () {
    let cli = Cli::parse();
    register_commands(cli);
}
