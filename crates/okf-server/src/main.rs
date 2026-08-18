use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "okf-server", version, about = "OKF REST API server")]
struct Cli {
    /// Directory containing the OKF bundle.
    #[arg(long, default_value = "./docs")]
    data: std::path::PathBuf,

    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:8080")]
    bind: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    if let Err(e) = okf_server::serve(cli.data, cli.bind).await {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
