use std::path::PathBuf;
use clap::Parser;
use starbase::tracing::info;
use starbase::{system, App, MainResult, State};

mod cli_args;
mod settings;
use cli_args::Cli;
use settings::Settings;

#[derive(Debug, State)]
pub struct Config(Settings);

#[derive(Debug, State)]
pub struct WorkspaceRoot(PathBuf);

#[derive(Debug, State)]
pub struct ArgMatches(clap::ArgMatches);

#[derive(Debug, State)]
pub struct CliArgs(Cli);

#[tokio::main]
async fn main() -> MainResult {
    App::setup_diagnostics();
    App::setup_tracing();

    let mut app = App::new();
    app.shutdown(finish);
    app.startup(load_config);
    app.startup(gather_cli_args);
    app.startup(preinit_workspace);
    app.run().await?;
    Ok(())
}

#[system]
async fn preinit_workspace(states: StatesMut) {
    // Set workspace_path to the current working directory of the executing application
    let workspace_path = std::env::current_dir().unwrap();
    states.set(WorkspaceRoot(workspace_path));
}

#[system]
async fn finish(state: StateRef<WorkspaceRoot>) {
    let workspace_path = state.0.to_str().unwrap();
    info!(val = workspace_path, "shutdown");
    // dbg!(state);
}

#[system]
async fn gather_cli_args(states: StatesMut) {
    let cli = Cli::parse();
    states.set(CliArgs(cli));
}

#[system]
async fn load_config(states: StatesMut) -> SystemResult {
    let settings = Settings::new()?;
    //TODO: do logging here
    //println!("settings: {:?}", settings);

    let config: Config = Config(settings);
    states.set::<Config>(config);

    ()
}
