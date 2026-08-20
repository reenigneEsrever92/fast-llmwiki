//! Unified CLI launcher for the OKF server, web UI, and semantic search.
//!
//! `fawi-cli` composes the startup logic exposed by `fawi-server`, `fawi-gui`, and
//! `fawi-search` into a single `okf` binary. Logging is initialized by the
//! binary, never by the library startup functions.

mod skills;

use clap::{Args, Parser, Subcommand};

/// The unified `okf` entry point.
#[derive(Debug, Parser)]
#[command(name = "okf", version, about = "Unified OKF launcher")]
pub struct Cli {
    /// Directory containing the OKF bundle.
    #[arg(long, default_value = "./docs")]
    pub data: std::path::PathBuf,

    /// Address to listen on.
    #[arg(long, default_value = "127.0.0.1:8080")]
    pub bind: String,

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
    /// Install the agent skills bundled in this binary.
    Install(InstallArgs),
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

#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Directory to install skills into.
    #[arg(long, default_value = ".agents/skills")]
    pub dir: std::path::PathBuf,
}

/// Dispatch the parsed CLI, running the selected component(s) until they exit
/// or a termination signal arrives.
pub async fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Command::Server(args)) => {
            tokio::select! {
                res = fawi_server::serve(args.data, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        Some(Command::Gui(args)) => {
            tokio::select! {
                res = fawi_gui::ssr::serve(args.api_base_url, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        Some(Command::Search(args)) => {
            tokio::select! {
                res = fawi_search::serve(args.data, args.bind) => res,
                _ = shutdown_signal() => Ok(()),
            }
        }
        Some(Command::Install(args)) => install_skills(&args.dir),
        None => run_all(cli.data, cli.bind).await,
    }
}

/// Install every skill embedded in the binary into `dir`, one `SKILL.md` per
/// nested `<dir>/<name>/` directory. Re-runs overwrite existing files.
fn install_skills(dir: &std::path::Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(dir)
        .map_err(|e| anyhow::anyhow!("failed to create {}: {e}", dir.display()))?;

    for skill in skills::SKILLS {
        let skill_dir = dir.join(skill.name);
        std::fs::create_dir_all(&skill_dir)
            .map_err(|e| anyhow::anyhow!("failed to create {}: {e}", skill_dir.display()))?;
        let dest = skill_dir.join("SKILL.md");
        std::fs::write(&dest, skill.content)
            .map_err(|e| anyhow::anyhow!("failed to write {}: {e}", dest.display()))?;
        println!("installed skill {} -> {}", skill.name, dest.display());
    }

    Ok(())
}

/// Start the REST API, web UI, and semantic search on a single socket.
///
/// The three routers are merged into one axum app so the web UI and its API
/// share an origin (which is what client-side navigation needs), and the whole
/// bundle is served by a single binary on a single port.
async fn run_all(data: std::path::PathBuf, bind: String) -> anyhow::Result<()> {
    tokio::select! {
        res = serve_all(data, bind) => res,
        _ = shutdown_signal() => Ok(()),
    }
}

async fn serve_all(data: std::path::PathBuf, bind: String) -> anyhow::Result<()> {
    // Open the bundle once and share it with the API and semantic-search crates.
    let bundle = fawi_storage::FsBundle::open(&data).await?;
    fawi_server::api::init_bundle(bundle.clone());
    fawi_search::api::init_bundle(bundle).await?;

    // SSR fetches the API over HTTP on this same socket, so point it at the
    // loopback address for the chosen port.
    let port = bind.rsplit(':').next().unwrap_or("8080");
    fawi_gui::api_client::set_api_base_url(format!("http://127.0.0.1:{port}"));

    let app = axum::Router::new()
        .merge(fawi_server::api::router())
        .merge(fawi_search::api::router())
        .merge(fawi_gui::ssr::router());

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind {bind}: {e}"))?;
    tracing::info!("OKF on http://{bind} (bundle: {})", data.display());
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("server error: {e}"))?;

    Ok(())
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

    #[test]
    fn parses_install_subcommand_with_default_dir() {
        let cli = Cli::try_parse_from(["okf", "install"]).unwrap();
        match cli.command {
            Some(Command::Install(args)) => {
                assert_eq!(args.dir, std::path::PathBuf::from(".agents/skills"));
            }
            _ => panic!("expected install subcommand"),
        }
    }

    #[test]
    fn parses_install_subcommand_with_dir() {
        let cli = Cli::try_parse_from(["okf", "install", "--dir", "/tmp/skills"]).unwrap();
        match cli.command {
            Some(Command::Install(args)) => {
                assert_eq!(args.dir, std::path::PathBuf::from("/tmp/skills"))
            }
            _ => panic!("expected install subcommand"),
        }
    }

    #[test]
    fn install_skills_writes_every_skill() {
        let dir = std::env::temp_dir().join(format!("fawi-cli-install-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        install_skills(&dir).unwrap();

        for skill in skills::SKILLS {
            let dest = dir.join(skill.name).join("SKILL.md");
            assert_eq!(std::fs::read_to_string(&dest).unwrap(), skill.content);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }
}
