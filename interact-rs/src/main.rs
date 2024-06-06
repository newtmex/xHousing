mod cli;
mod config;
mod helpers;
mod interact;
mod state;
mod types;
mod constants;

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
        },
    }
}
