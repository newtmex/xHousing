use clap::{Args, Parser};

#[derive(PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub enum Cli {
    #[command(name = "deploy-contracts", about = "Deploy contracts")]
    DeployContracts,

    #[command(name = "upgrade-contracts", about = "Upgrade contracts")]
    UpgradeContracts,

    #[command(
        name = "start-ico",
        about = "Deploys contracts if not already, then starts ICO"
    )]
    StartICO,

    #[command(name = "deploy-x-project", about = "Deploys an XProject contract")]
    DeployXProject(DeployXProjectArgs),
}

#[derive(Debug, PartialEq, Eq, Clone, Args)]
pub struct DeployXProjectArgs {
    pub project_id: usize,
    pub name: String,
    pub funding_goal: u64,
}
