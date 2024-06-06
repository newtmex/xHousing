use clap::Parser;

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
}
