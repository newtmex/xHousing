mod cli;
mod config;
mod constants;
mod helpers;
mod interact;
mod state;
mod types;

use clap::Parser;
use cli::Cli;
use interact::Interact;
use multiversx_sc_snippets::imports::*;

#[tokio::main]
async fn main() {
    let mut interact = Interact::init().await;
    let command = Cli::parse();

    match command {
        Cli::DeployContracts => {
            interact.deploy_contracts().await;
        }
        Cli::UpgradeContracts => todo!(),
        Cli::StartICO => {
            interact.start_ico().await;
        }
        Cli::DeployXProject(args) => {
            interact
                .deploy_x_project(args.project_id, &args.name, args.funding_goal)
                .await;
        }
    }
}
