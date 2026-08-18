//! Unified CLI launcher for the OKF server, web UI, and semantic search.
//!
//! `okf-cli` composes the startup logic exposed by `okf-server`, `okf-gui`, and
//! `okf-search` into a single `okf` binary. Logging is initialized by the
//! binary, never by the library startup functions.

use clap::{Args, Parser, Subcommand};

/// The unified `okf` entry point.
#[derive(Debug, Parser)]
#[command(name = "okf", version, about = "Unified OKF launcher")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start the REST API only.
    Server(ServerArgs),
    /// Start the web UI only.
    Gui(GuiArgs),
    /// Start the semantic search API only.
    Search(SearchArgs),
}

#[derive(Debug, Args)]
pub struct ServerArgs {
    /// Directory containing the OKF bundle.
    #[arg(long, default_value = "./docs")]
    pub data: std::path::PathBuf,

    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub bind: String,
}

#[derive(Debug, Args)]
pub struct GuiArgs {
    /// Base URL of the OKF REST API.
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    pub api_base_url: String,

    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:8081")]
    pub bind: String,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Directory containing the OKF bundle.
    #[arg(long, default_value = "./docs")]
    pub data: std::path::PathBuf,

    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:8082")]
    pub bind: String,
}

/// Dispatch the parsed CLI, running the selected component(s) until they exit
/// or a termination signal arrives.
pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Command::Server(args)) => {
            tokio::select! {
                res = okf_server::serve(args.data, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        Some(Command::Gui(args)) => {
            tokio::select! {
                res = okf_gui::ssr::serve(args.api_base_url, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        Some(Command::Search(args)) => {
            tokio::select! {
                res = okf_search::serve(args.data, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        None => run_both().await,
    }
}

/// Start the server, GUI, and semantic search service concurrently, wiring the
/// GUI's API base URL to the server's bind address. All are stopped on
/// SIGINT/SIGTERM.
async fn run_both() -> anyhow::Result<()> {
    let data = std::path::PathBuf::from("./docs");
    let server_bind = "127.0.0.1:8080".to_string();
    let gui_bind = "127.0.0.1:8081".to_string();
    let api_base_url = format!("http://{server_bind}");

    let server = okf_server::serve(data.clone(), server_bind);
    let gui = okf_gui::ssr::serve(api_base_url, gui_bind);
    let search = okf_search::serve(data, "127.0.0.1:8082".to_string());

    tokio::select! {
        res = server => res,
        res = gui => res,
        res = search => res,
        _ = shutdown_signal() => Ok(()),
    }
}

/// Resolve to completion when the process receives SIGINT or SIGTERM (Ctrl-C on
/// non-Unix platforms).
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to install SIGINT handler");
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = sigint.recv() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl-C handler");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_server_subcommand() {
        let cli = Cli::try_parse_from(["okf", "server", "--bind", "127.0.0.1:9000"]).unwrap();
        match cli.command {
            Some(Command::Server(args)) => assert_eq!(args.bind, "127.0.0.1:9000"),
            _ => panic!("expected server subcommand"),
        }
    }

    #[test]
    fn parses_gui_subcommand_with_defaults() {
        let cli = Cli::try_parse_from(["okf", "gui"]).unwrap();
        match cli.command {
            Some(Command::Gui(args)) => {
                assert_eq!(args.bind, "127.0.0.1:8081");
                assert_eq!(args.api_base_url, "http://127.0.0.1:8080");
            }
            _ => panic!("expected gui subcommand"),
        }
    }

    #[test]
    fn parses_search_subcommand_with_defaults() {
        let cli = Cli::try_parse_from(["okf", "search"]).unwrap();
        match cli.command {
            Some(Command::Search(args)) => {
                assert_eq!(args.bind, "127.0.0.1:8082");
                assert_eq!(args.data, std::path::PathBuf::from("./docs"));
            }
            _ => panic!("expected search subcommand"),
        }
    }

    #[test]
    fn defaults_to_no_subcommand() {
        let cli = Cli::try_parse_from(["okf"]).unwrap();
        assert!(cli.command.is_none());
    }
}
