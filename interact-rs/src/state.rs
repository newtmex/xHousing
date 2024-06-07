use std::{io::Write, path::Path};

use multiversx_sc_snippets::imports::Bech32Address;
use serde::{Deserialize, Serialize};

use crate::helpers::toml_into;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub state_file: String,
    pub coinbase_addr: Option<Bech32Address>,
    pub x_housing_addr: Option<Bech32Address>,
    pub x_project_funding_addr: Option<Bech32Address>,
    pub x_project_template_addr: Option<Bech32Address>,
    pub xht_id: Option<String>,
    pub lk_xht_id: Option<String>,
}

impl State {
    fn new(state_file: &str) -> Self {
        let mut state = State::default();
        state.state_file = state_file.into();

        state
    }

    pub fn load_state(state_file: &str) -> Self {
        if Path::new(state_file).exists() {
            toml_into(state_file)
        } else {
            Self::new(state_file)
        }
    }
}

// Serializes state to file
impl Drop for State {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(&self.state_file).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}
