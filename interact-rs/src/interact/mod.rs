mod coinbase;
mod x_housing;
mod x_project;
mod x_project_funding;

use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Result};
use multiversx_sc::{
    imports::OptionalValue,
    types::{Address, ManagedAddress},
};
use multiversx_sc_snippets::{
    imports::{
        BytesValue, ExpectStatus, InterpretableFrom, InterpreterContext, ReturnsNewTokenIdentifier,
        StaticApi,
    },
    sdk::wallet::Wallet,
    Interactor, InteractorPrepareAsync,
};
use reqwest::Client;
use utils::contracts_proxy::{
    coinbase_proxy::CoinbaseProxy, x_project_funding_proxy::XProjectFundingProxy,
};

use crate::{
    config::Config, constants::ESDT_REG_COST, helpers::toml_into, state::State, types::CargoFile,
};

const GET_NETWORK_STATUS_ENDPOINT: &str = "network/status";
pub const METACHAIN_SHARD_ID: u32 = 0xFFFFFFFF;

pub struct Interact {
    state: State,
    interactor: Interactor,
    contracts_owner: Address,
    contracts_owner_wallet: Wallet,
    config: Config,
    client: Client,
}

impl Interact {
    pub async fn init() -> Self {
        let config = Config::load_config();
        let mut interactor = Interactor::new(&config.gateway).await;

        let interaction_folder = format!(
            "./interaction/{}/{}",
            interactor.network_config.chain_id, interactor.network_config.start_time
        );
        let state_file = format!("{interaction_folder}/state.toml",);
        // Create state file
        fs::create_dir_all(Path::new(&state_file).parent().unwrap())
            .unwrap_or_else(|err| assert!(err.to_string().contains("File exists"), "{}", err));

        let contracts_owner_wallet = Wallet::from_pem_file(&config.contracts_owner_pem)
            .expect("Contracts owner pem file not found");
        let contracts_owner = interactor.register_wallet(contracts_owner_wallet);

        Self {
            state: State::load_state(&state_file),
            interactor,
            config,
            contracts_owner,
            contracts_owner_wallet,
            client: Client::new(),
        }
    }

    fn build_contract(&self, contract_rel_path: &str) -> BytesValue {
        let canonical_path = std::fs::canonicalize(contract_rel_path).unwrap();

        assert!(
            canonical_path.exists(),
            "Path: {} not found",
            canonical_path.to_str().unwrap()
        );

        let build_dir = canonical_path.join("meta");
        let build_dir = build_dir.to_str().unwrap();

        // get contract name
        let contract_cargo_toml = canonical_path.join("Cargo.toml");
        let contract_cargo_toml = contract_cargo_toml.to_str().unwrap();
        let contract_name = toml_into::<CargoFile>(contract_cargo_toml)
            .package
            .unwrap()
            .name;

        // Find workspace dir
        let mut target_dir = String::new();
        let mut depth = 0u32;
        loop {
            if depth >= 5 {
                break;
            }

            let mut backs = String::new();
            for _ in 0..=depth {
                backs.push_str("../")
            }
            let folder = canonical_path.join(backs);
            let folder = folder.to_str().unwrap();
            let cargo_file = format!("{folder}Cargo.toml");
            if Path::new(&cargo_file).exists()
                && toml_into::<CargoFile>(&cargo_file).workspace.is_some()
            {
                target_dir = format!("{folder}target");
                break;
            }

            depth += 1;
        }
        assert!(
            !target_dir.is_empty(),
            "target dir empty; {}",
            contract_rel_path
        );

        let mut cargo = Command::new("cargo");
        let build_cmd = cargo
            .args(["run", "build", "--target-dir", &target_dir])
            .current_dir(build_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let exit_status = build_cmd.spawn().unwrap().wait().unwrap();
        assert!(exit_status.success(), "{contract_name} build failed");

        let output_file = canonical_path.join(format!("output/{contract_name}.mxsc.json"));
        let output_file = output_file.to_str().unwrap();

        BytesValue::interpret_from(
            &format!("mxsc:{output_file}")[..],
            &InterpreterContext::default(),
        )
    }

    pub async fn deploy_contracts(&mut self) {
        self.deploy_coinbase().await;
        self.register_xht().await;

        self.deploy_x_project_funding().await;
        self.deploy_x_housing().await;
        self.deploy_x_project_template().await;
    }

    pub async fn start_ico(&mut self) {
        self.deploy_contracts().await;

        // TODO
        // let mut find = Command::new("find");
        // let find_rslt = find
        //     .args(["contracts", "-name", "*mxsc.json"])
        //     .current_dir("../").output().unwrap().stdout;

        let x_housing_addr = self.get_x_housing_addr();
        let coinbase_addr = self.get_coinbase_addr();
        let x_project_funding_addr = self.get_x_project_funding_addr();
        let x_project_template_addr = self.get_x_project_template_addr();
        let contracts_owner = &self.contracts_owner;

        if self.state.lk_xht_id.is_none() {
            // TODO req on testnet fails
            let lk_xht_id = self
                .interactor
                .tx()
                .to(&x_project_funding_addr)
                .gas(500_000_000)
                .from(contracts_owner)
                .typed(XProjectFundingProxy)
                .register_lk_xht_token()
                .egld(ESDT_REG_COST)
                .returns(ReturnsNewTokenIdentifier)
                .prepare_async()
                .run()
                .await;

            self.state.lk_xht_id = Some(lk_xht_id);
        }

        let current_timestamp = self
            .get_latest_hyper_block_status(true)
            .await
            .unwrap()
            .status
            .current_timestamp;
        self.interactor
            .tx()
            .to(&coinbase_addr)
            .from(contracts_owner)
            .gas(500_000_000)
            .typed(CoinbaseProxy)
            .start_ico(
                &x_project_funding_addr,
                x_project_template_addr,
                &x_housing_addr,
                "EGLD",
                2u64 * 10u64.pow(15),
                current_timestamp + 100_000,
            )
            .returns(ExpectStatus(0))
            .prepare_async()
            .run()
            .await;

        self.interactor
            .tx()
            .to(&x_project_funding_addr)
            .from(contracts_owner)
            .gas(500_000_000)
            .typed(XProjectFundingProxy)
            .set_x_project_token(1usize, "UloGold")
            .egld(ESDT_REG_COST)
            .returns(ExpectStatus(0))
            .prepare_async()
            .run()
            .await;

        self.interactor
            .tx()
            .from(contracts_owner)
            .gas(500_000_000)
            .to(coinbase_addr)
            .typed(CoinbaseProxy)
            .feed_x_housing(OptionalValue::<ManagedAddress<StaticApi>>::Some(
                x_housing_addr.into_address().into(),
            ))
            .returns(ExpectStatus(0))
            .prepare_async()
            .run()
            .await;
    }

    pub async fn deploy_x_project(&mut self, project_id: usize, name: &str, funding_goal: u64) {
        let x_project_funding_addr = self.get_x_project_funding_addr();
        let contracts_owner = &self.contracts_owner;
        let current_timestamp = self
            .get_latest_hyper_block_status(true)
            .await
            .unwrap()
            .status
            .current_timestamp;

        self.interactor
            .tx()
            .to(&x_project_funding_addr)
            .from(contracts_owner)
            .gas(500_000_000)
            .typed(XProjectFundingProxy)
            .deploy_x_project(
                "EGLD",
                funding_goal * 10u64.pow(12),
                current_timestamp + 100_000,
            )
            .prepare_async()
            .run()
            .await;

        self.interactor
            .tx()
            .to(&x_project_funding_addr)
            .from(contracts_owner)
            .gas(500_000_000)
            .typed(XProjectFundingProxy)
            .set_x_project_token(project_id, name)
            .egld(ESDT_REG_COST)
            .returns(ExpectStatus(0))
            .prepare_async()
            .run()
            .await;
    }

    fn get_endpoint(&self, endpoint: &str) -> String {
        format!("{}/{}", &self.config.gateway, endpoint)
    }

    // get_latest_hyper_block_nonce retrieves the latest hyper block (metachain) nonce from the network
    pub async fn get_latest_hyper_block_status(
        &self,
        with_metachain: bool,
    ) -> Result<crate::types::NetworkStatusData> {
        let mut endpoint = GET_NETWORK_STATUS_ENDPOINT.to_string();

        if with_metachain {
            endpoint = format!("{GET_NETWORK_STATUS_ENDPOINT}/{METACHAIN_SHARD_ID}");
        }

        let endpoint = self.get_endpoint(endpoint.as_str());

        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<crate::types::NetworkStatusResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b),
        }
    }
}
