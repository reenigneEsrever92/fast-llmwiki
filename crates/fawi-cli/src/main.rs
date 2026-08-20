use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = fawi_cli::Cli::parse();

    if let Err(e) = fawi_cli::run(cli).await {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
