pub mod network_status;

pub use network_status::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CargoFile {
    pub package: Option<CargoFilePackage>,
    pub workspace: Option<CargoFileWorkspace>,
}

#[derive(Debug, Deserialize)]
pub struct CargoFilePackage {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CargoFileWorkspace {
    // members: Option<Vec<String>>,
}
