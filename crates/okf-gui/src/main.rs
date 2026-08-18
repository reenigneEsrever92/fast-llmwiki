#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use clap::Parser;
    use tracing_subscriber::EnvFilter;

    #[derive(Debug, Parser)]
    #[command(name = "okf-gui", version, about = "OKF web UI (queries the OKF REST API)")]
    struct Cli {
        /// Base URL of the OKF REST API.
        #[arg(long, default_value = "http://127.0.0.1:8080")]
        api_base_url: String,

        /// Address to listen on.
        #[arg(long, default_value = "127.0.0.1:8081")]
        bind: String,
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    if let Err(e) = okf_gui::ssr::serve(cli.api_base_url, cli.bind).await {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // No client-side entry point; hydration is in `lib.rs`.
}
