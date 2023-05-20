use clap::Parser;

use ens_client::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    // println!("{:?}", cli);
    cli.run().await.unwrap();
}
